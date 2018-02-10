use ingredients::ingredient::*;
use contracts::*;
use book_keeper::*;
use std::vec::Vec;
use std::string::String;
use std::collections::HashMap;
use tools::*;
extern crate serde_json;

pub struct Reference {
    type_: EntityType,
    optional_vals: Vec<FieldValue>
}

struct ReferenceConfig {
    type_: String,
    optional_values: Vec<serde_json::Value>,
}

impl Reference {
    pub fn new(type_: EntityType) -> Reference {
        Reference {
            type_,
            optional_vals: vec![]
        }
    }

    /// Specify which values should be treated as optional
    pub fn optional(&mut self, optional_vals: Vec<FieldValue>) -> &mut Self {
        self.optional_vals = optional_vals.clone();

        self
    }
}

impl Ingredient<ReferenceConfig> for Reference {
    /// Get all dependencies of this ingredient
    fn get_deps(&self, value: FieldValue, row: Row, circular: bool) -> Vec<Dep> {
        for v in &self.optional_vals {
            if *v == value {
                return vec![];
            }
        }

        field_value_to_id(value)
            .map(|v| vec![(self.type_.clone(), v)])
            .unwrap_or(vec![])
    }

    /// Let the ingredient determine the value of the field to store in a serialization
    fn serialize(&self, value: FieldValue, row: Row, books: &BookKeeper, circular: bool) -> Option<FieldValue> {
        for v in &self.optional_vals {
            if *v == value {
                return Some(value);
            }
        }

        field_value_to_id(value)
            .and_then(|id| books.resolve_id(self.type_.clone(), id, false))
            .map(id_to_field_value)
    }

    /// Let the ingredient determine the value of the field to insert into the database when deserializing
    fn deserialize(&self, value: FieldValue, row: Row, books: &BookKeeper) -> Option<DeserializedValue> {
        for v in &self.optional_vals {
            if *v == value {
                return Some(DeserializedValue::new(vec![], value));
            }
        }

        field_value_to_id(value)
            .and_then(|id| books.resolve_id(self.type_.clone(), id.clone(), false)
                .map(|resolved| (id_to_field_value(resolved), id))
            )
            .map(|(resolved, id)| DeserializedValue::new(vec![(self.type_.clone(), id)], resolved))
    }

    /// Should return an array with fields required to be able to UPDATE a row
    fn get_required_extra_fields(&self) -> Vec<String> {
        vec![]
    }

    /// Turn the ingredient into a config
    fn to_config(&self) -> IngredientConfig<ReferenceConfig> {
        IngredientConfig {
            type_: "REF",
            config: ReferenceConfig {
                type_: self.type_.clone(),
                optional_values: self.optional_vals
                    .iter()
                    .map(field_value_to_serde_value)
                    .collect(),
            }
        }
    }

    /// Create the ingredient from a config
    fn from_config(config: IngredientConfig<ReferenceConfig>) -> Self {
        Reference {
            type_: config.config.type_.to_string(),
            optional_vals: config.config.optional_values
                .iter()
                .map(serde_value_to_field_value)
                .collect(),
        }
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
        fn resolve_id(&self, type_: EntityType, id: Id, authoritative: bool) -> Option<Id> {
            Some(Id::Uuid(String::from("MOCK")))
        }
        fn reset(&mut self) { unimplemented!() }
    }

    #[test]
    fn it_gets_deps() {
        let mut r = Reference::new(String::from("foos"));

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
        let mut r = Reference::new(String::from("foos"));
        let b = BookKeeperMock::new();

        let o1 = r.serialize(FieldValue::Int(123), HashMap::new(), &b, false);

        assert!(o1.is_some());
        let serialized1 = o1.unwrap();

        assert_eq!(FieldValue::String(String::from("MOCK")), serialized1);

        let o2 = r.serialize(FieldValue::Null, HashMap::new(), &b, false);

        assert!(o2.is_none());

        r.optional(vec![FieldValue::Null]);

        let o3 = r.serialize(FieldValue::Null, HashMap::new(), &b, false);

        assert!(o3.is_some());
        let serialized3 = o3.unwrap();

        assert_eq!(FieldValue::Null, serialized3);
    }

    #[test]
    fn it_deserializes()
    {
        let mut r = Reference::new(String::from("foos"));
        let b = BookKeeperMock::new();

        let o1 = r.deserialize(FieldValue::Int(123), HashMap::new(), &b);

        assert!(o1.is_some());
        let deserialized1 = o1.unwrap();

        assert_eq!(1, deserialized1.deps().len());
        assert_eq!((String::from("foos"), Id::Int(123)), deserialized1.deps()[0]);
        assert_eq!(FieldValue::String(String::from("MOCK")), deserialized1.value());
    }
}