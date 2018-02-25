extern crate serde_json;

use ingredients::ingredient::*;
use contracts::*;
use book_keeper::*;
use std::vec::Vec;
use std::string::String;
use std::collections::HashMap;
use tools::*;

#[derive(Debug, Serialize, Deserialize)]
struct RawConfig {
    pub value: FieldValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Raw {
    #[serde(rename="type")]
    type_: String,
    config: RawConfig,
}

impl Raw {
    pub fn new(value: FieldValue) -> Raw { Raw { type_: "RAW".to_string(), config: RawConfig { value } } }
}

impl Ingredient for Raw {
    /// Get all dependencies of this ingredient
    fn get_deps(&self, _value: &FieldValue, _row: &Row, _circular: bool) -> Vec<Dep> {
        vec![]
    }

    /// Let the ingredient determine the value of the field to store in a serialization
    fn snapper_serialize(&self, _value: &FieldValue, _row: &Row, _books: &BookKeeper, _circular: bool) -> Option<FieldValue> {
        Some(self.config.value.clone())
    }

    /// Let the ingredient determine the value of the field to insert into the database when deserializing
    fn snapper_deserialize(&self, _value: &FieldValue, _row: &Row, _books: &BookKeeper) -> Option<DeserializedValue> {
        Some(DeserializedValue::new(vec![], self.config.value.clone()))
    }

    /// Should return an array with fields required to be able to UPDATE a row
    fn get_required_extra_fields(&self) -> Vec<String> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct BookKeeperMock {}

    impl BookKeeperMock {
        pub fn new() -> BookKeeperMock { BookKeeperMock {} }
    }

    impl BookKeeper for BookKeeperMock {
        fn resolve_id(&self, _etype: EntityType, _id: Id, _authoritative: bool) -> Option<Id> { unimplemented!() }
        fn reset(&mut self) { unimplemented!() }
    }

    #[test]
    fn it_gets_deps() {
        let r = Raw::new(FieldValue::Int(123));

        assert_eq!(0, r.get_deps(&FieldValue::Null, &HashMap::new(), false).len());
    }

    #[test]
    fn it_serializes() {
        let r = Raw::new(FieldValue::Int(123));
        let b = BookKeeperMock::new();

        assert_eq!(Some(FieldValue::Int(123)), r.snapper_serialize(&FieldValue::Null, &HashMap::new(), &b, false));
    }

    #[test]
    fn it_deserializes() {
        let r = Raw::new(FieldValue::Int(123));
        let b = BookKeeperMock::new();

        let o = r.snapper_deserialize(&FieldValue::Null, &HashMap::new(), &b);

        assert!(o.is_some());
        let d = o.unwrap();

        assert_eq!(FieldValue::Int(123), d.value());
        assert_eq!(0, d.deps().len());
    }

    #[test]
    fn it_gets_required_extra_fields() {
        let r = Raw::new(FieldValue::Int(123));

        assert_eq!(0, r.get_required_extra_fields().len());
    }
}