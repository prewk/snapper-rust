use ingredients::ingredient::*;
use contracts::*;
use book_keeper::*;
use std::vec::Vec;
use std::string::String;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct ValueConfig {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Value {
    #[serde(rename="type")]
    type_: String,
    config: ValueConfig,
}

impl Value {
    pub fn new() -> Value { Value { type_: "VALUE".to_string(), config: ValueConfig {} } }
}

impl Ingredient for Value {
    /// Get all dependencies of this ingredient
    fn get_deps(&self, _value: &FieldValue, _row: &Row, _circular: bool) -> Vec<Dep> {
        vec![]
    }

    /// Let the ingredient determine the value of the field to store in a serialization
    fn snapper_serialize(&self, value: &FieldValue, _row: &Row, _books: &BookKeeper, _circular: bool) -> Option<FieldValue> {
        Some(value.clone())
    }

    /// Let the ingredient determine the value of the field to insert into the database when deserializing
    fn snapper_deserialize(&self, value: &FieldValue, _row: &Row, _books: &BookKeeper) -> Option<DeserializedValue> {
        Some(DeserializedValue::new(vec![], value.clone()))
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
        let v = Value::new();

        assert_eq!(0, v.get_deps(&FieldValue::Null, &HashMap::new(), false).len());
        assert_eq!(0, v.get_deps(&FieldValue::Int(123), &HashMap::new(), false).len());
        assert_eq!(0, v.get_deps(&FieldValue::String(String::from("Foo")), &HashMap::new(), false).len());
    }

    #[test]
    fn it_serializes() {
        let v = Value::new();
        let b = BookKeeperMock::new();

        assert_eq!(Some(FieldValue::Int(123)), v.snapper_serialize(&FieldValue::Int(123), &HashMap::new(), &b, false));
    }

    #[test]
    fn it_deserializes() {
        let v = Value::new();
        let b = BookKeeperMock::new();

        let o = v.snapper_deserialize(&FieldValue::Int(123), &HashMap::new(), &b);

        assert!(o.is_some());
        let d = o.unwrap();

        assert_eq!(FieldValue::Int(123), d.value());
        assert_eq!(0, d.deps().len());
    }
}