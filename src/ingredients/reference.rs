use ingredients::ingredient::*;
use contracts::*;
use book_keeper::*;
use std::vec::Vec;
use std::string::String;
use std::collections::HashMap;
use tools::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct RefConfig {
    #[serde(rename="type")]
    pub type_: EntityType,
    pub optional_values: Vec<FieldValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reference {
    #[serde(rename="type")]
    type_: String,
    config: RefConfig,
}

impl Reference {
    pub fn new(type_: EntityType, optional_values: Vec<FieldValue>) -> Reference {
        Reference {
            type_: "REF".to_string(),
            config: RefConfig {
                type_,
                optional_values,
            },
        }
    }

    /// Specify which values should be treated as optional
    pub fn optional(&mut self, optional_values: Vec<FieldValue>) -> &mut Self {
        self.config.optional_values = optional_values.clone();

        self
    }
}

impl Ingredient for Reference {
    /// Get all dependencies of this ingredient
    fn get_deps(&self, value: FieldValue, _row: Row, _circular: bool) -> Vec<Dep> {
        for v in &self.config.optional_values {
            if *v == value {
                return vec![];
            }
        }

        field_value_to_id(value)
            .map(|v| vec![(self.config.type_.clone(), v)])
            .unwrap_or(vec![])
    }

    /// Let the ingredient determine the value of the field to store in a serialization
    fn snapper_serialize(&self, value: FieldValue, _row: Row, books: &BookKeeper, _circular: bool) -> Option<FieldValue> {
        for v in &self.config.optional_values {
            if *v == value {
                return Some(value);
            }
        }

        field_value_to_id(value)
            .and_then(|id| books.resolve_id(self.config.type_.clone(), id, false))
            .map(id_to_field_value)
    }

    /// Let the ingredient determine the value of the field to insert into the database when deserializing
    fn snapper_deserialize(&self, value: FieldValue, _row: Row, books: &BookKeeper) -> Option<DeserializedValue> {
        for v in &self.config.optional_values {
            if *v == value {
                return Some(DeserializedValue::new(vec![], value));
            }
        }

        field_value_to_id(value)
            .and_then(|id| books.resolve_id(self.config.type_.clone(), id.clone(), false)
                .map(|resolved| (id_to_field_value(resolved), id))
            )
            .map(|(resolved, id)| DeserializedValue::new(vec![(self.config.type_.clone(), id)], resolved))
    }

    /// Should return an array with fields required to be able to UPDATE a row
    fn get_required_extra_fields(&self) -> Vec<String> {
        vec![]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct BookKeeperMock {}

    impl BookKeeperMock {
        pub fn new() -> BookKeeperMock { BookKeeperMock {} }
    }

    impl BookKeeper for BookKeeperMock {
        fn resolve_id(&self, _type_: EntityType, _id: Id, _authoritative: bool) -> Option<Id> {
            Some(Id::Uuid(String::from("MOCK")))
        }
        fn reset(&mut self) { unimplemented!() }
    }

    #[test]
    fn it_gets_deps() {
        let mut r = Reference::new(String::from("foos"), vec![]);

        let deps1 = r.get_deps(FieldValue::Int(123), HashMap::new(), false);

        assert_eq!(1, deps1.len());
        assert_eq!((String::from("foos"), Id::Int(123)), deps1[0]);

        let deps2 = r.get_deps(FieldValue::Null, HashMap::new(), false);

        assert_eq!(0, deps2.len());

        r.optional(vec![FieldValue::Int(123)]);

        let deps3 = r.get_deps(FieldValue::Int(123), HashMap::new(), false);

        assert_eq!(0, deps3.len());
    }

    #[test]
    fn it_serializes()
    {
        let mut r = Reference::new(String::from("foos"), vec![]);
        let b = BookKeeperMock::new();

        let o1 = r.snapper_serialize(FieldValue::Int(123), HashMap::new(), &b, false);

        assert!(o1.is_some());
        let serialized1 = o1.unwrap();

        assert_eq!(FieldValue::String(String::from("MOCK")), serialized1);

        let o2 = r.snapper_serialize(FieldValue::Null, HashMap::new(), &b, false);

        assert!(o2.is_none());

        r.optional(vec![FieldValue::Null]);

        let o3 = r.snapper_serialize(FieldValue::Null, HashMap::new(), &b, false);

        assert!(o3.is_some());
        let serialized3 = o3.unwrap();

        assert_eq!(FieldValue::Null, serialized3);
    }

    #[test]
    fn it_deserializes()
    {
        let r = Reference::new(String::from("foos"), vec![]);
        let b = BookKeeperMock::new();

        let o1 = r.snapper_deserialize(FieldValue::Int(123), HashMap::new(), &b);

        assert!(o1.is_some());
        let deserialized1 = o1.unwrap();

        assert_eq!(1, deserialized1.deps().len());
        assert_eq!((String::from("foos"), Id::Int(123)), deserialized1.deps()[0]);
        assert_eq!(FieldValue::String(String::from("MOCK")), deserialized1.value());
    }
}