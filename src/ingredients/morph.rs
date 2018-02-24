use ingredients::ingredient::*;
use tools::{field_value_to_id, id_to_field_value};
use contracts::*;
use book_keeper::*;
use std::vec::Vec;
use std::string::String;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct MorphMapper {
    morph_map: HashMap<FieldValue, EntityType>
}

impl MorphMapper {
    /// Help Morph find its dependency
    pub fn get_deps(&self, morph_type: &FieldValue, value: &FieldValue) -> Vec<Dep> {
        self.morph_map.get(morph_type)
            .and_then(|etype: &EntityType| {
                field_value_to_id(value.clone())
                    .map(|id| vec![(etype.clone(), id)])
            })
            .unwrap_or(vec![])
    }

    /// Help Morph resolve its value
    pub fn resolve(&self, morph_type: &FieldValue, value: &FieldValue, books: &BookKeeper) -> Option<Id> {
        self.morph_map.get(morph_type)
            .and_then(|etype: &EntityType| {
                field_value_to_id(value.clone())
                    .and_then(|id| {
                        books.resolve_id(etype.clone(), id, false)
                    })
            })
    }

    /// Resolve a morph type into a dependency type
    pub fn resolve_type(&self, morph_type: &FieldValue) -> Option<EntityType> {
        self.morph_map.get(morph_type)
            .map(|etype: &EntityType| {
                etype.clone()
            })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct MorphConfig {
    field: String,
    morph_mapper: MorphMapper,
    optional_values: Vec<FieldValue>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Morph {
    #[serde(rename="type")]
    type_: String,
    config: MorphConfig,
}

impl Morph {
    pub fn new(field: String, morph_map: HashMap<FieldValue, String>, optional_values: Vec<FieldValue>) -> Morph {
        Morph {
            type_: "MORPH".to_string(),
            config: MorphConfig {
                field,
                morph_mapper: MorphMapper {
                    morph_map,
                },
                optional_values,
            }
        }
    }

    /// Specify which values should be treated as optional
    pub fn optional(&mut self, optional_values: Vec<FieldValue>) -> &mut Self {
        self.config.optional_values = optional_values.clone();

        self
    }

    /// Get the morph type for the ingredient
    fn get_morph_type(&self, value: &FieldValue, row: &Row) -> Option<FieldValue> {
        row.get(&self.config.field)
            .and_then(|morph_type: &FieldValue| {
                for v in &self.config.optional_values {
                    if v == value || v == morph_type {
                        return None;
                    }
                }

                Some(morph_type.clone())
            })
    }
}

impl Ingredient for Morph {
    /// Get all dependencies of this ingredient
    fn get_deps(&self, value: FieldValue, row: Row, _circular: bool) -> Vec<Dep> {
        self.get_morph_type(&value, &row)
            .map(|morph_type| {
                self.config.morph_mapper.get_deps(&morph_type, &value)
            })
            .unwrap_or(vec![])
    }

    /// Let the ingredient determine the value of the field to store in a serialization
    fn snapper_serialize(&self, value: FieldValue, row: Row, books: &BookKeeper, _circular: bool) -> Option<FieldValue> {
        self.get_morph_type(&value, &row)
            .and_then(|morph_type| {
                self.config.morph_mapper.resolve(&morph_type, &value, books)
                    .map(|id| {
                        id_to_field_value(id)
                    })
            })
    }

    /// Let the ingredient determine the value of the field to insert into the database when deserializing
    fn snapper_deserialize(&self, value: FieldValue, row: Row, books: &BookKeeper) -> Option<DeserializedValue> {
        self.get_morph_type(&value, &row)
            .and_then(|morph_type| {
                let ref_type = self.config.morph_mapper.resolve_type(&morph_type);
                let id = self.config.morph_mapper.resolve(&morph_type, &value, books);

                let f = |(ref_type, id): (&EntityType, &Id)| {
                    DeserializedValue::new(vec![(ref_type.clone(), id.clone())], value.clone())
                };

                ref_type.iter().zip(id.iter()).map(f).next()
            })
    }

    /// Should return an array with fields required to be able to UPDATE a row
    fn get_required_extra_fields(&self) -> Vec<String> {
        vec![self.config.field.clone()]
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
        let mut morph_map: HashMap<FieldValue, EntityType> = HashMap::new();
        morph_map.insert(FieldValue::String(String::from("BAR")), String::from("bars"));
        morph_map.insert(FieldValue::String(String::from("BAZ")), String::from("bazes"));

        let m = Morph::new(String::from("fooable_type"), morph_map, vec![]);

        let mut row: Row = HashMap::new();
        row.insert("fooable_type".to_string(), FieldValue::String(String::from("BAR")));

        let deps1 = m.get_deps(FieldValue::Int(123), row.clone(), false);

        assert_eq!(1, deps1.len());

        let deps2 = m.get_deps(FieldValue::Null, row.clone(), false);

        assert_eq!(0, deps2.len());

        let deps3 = m.get_deps(FieldValue::Int(123), HashMap::new(), false);

        assert_eq!(0, deps3.len());
    }

    #[test]
    fn it_serializes()
    {
        let b = BookKeeperMock::new();

        let mut morph_map: HashMap<FieldValue, EntityType> = HashMap::new();
        morph_map.insert(FieldValue::String(String::from("BAR")), String::from("bars"));
        morph_map.insert(FieldValue::String(String::from("BAZ")), String::from("bazes"));

        let mut m = Morph::new(String::from("fooable_type"), morph_map, vec![]);

        let mut row: Row = HashMap::new();
        row.insert("fooable_type".to_string(), FieldValue::String(String::from("BAR")));

        let o1 = m.snapper_serialize(FieldValue::Int(123), row.clone(), &b, false);

        assert!(o1.is_some());
        let serialized1 = o1.unwrap();

        assert_eq!(FieldValue::String(String::from("MOCK")), serialized1);

        let o2 = m.snapper_serialize(FieldValue::Null, row.clone(), &b, false);

        assert!(o2.is_none());

        m.optional(vec![FieldValue::Int(123)]);

        let o2 = m.snapper_serialize(FieldValue::Int(123), row.clone(), &b, false);

        assert!(o2.is_none());
    }

    #[test]
    fn it_deserializes()
    {
        let b = BookKeeperMock::new();

        let mut morph_map: HashMap<FieldValue, EntityType> = HashMap::new();
        morph_map.insert(FieldValue::String(String::from("BAR")), String::from("bars"));
        morph_map.insert(FieldValue::String(String::from("BAZ")), String::from("bazes"));

        let m = Morph::new(String::from("fooable_type"), morph_map, vec![]);

        let mut row: Row = HashMap::new();
        row.insert("fooable_type".to_string(), FieldValue::String(String::from("BAR")));

        let o1 = m.snapper_deserialize(FieldValue::Int(123), row.clone(), &b);

        assert!(o1.is_some());
        let deserialized1 = o1.unwrap();

        assert_eq!(1, deserialized1.deps().len());
        assert_eq!((String::from("bars"), Id::Uuid(String::from("MOCK"))), deserialized1.deps()[0]);
        assert_eq!(FieldValue::Int(123), deserialized1.value());
    }
}