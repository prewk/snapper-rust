use ingredients::ingredient::*;
use contracts::*;
use book_keeper::*;
use std;

pub struct Value {}

impl Value {
    pub fn new() -> Value { Value {} }
}

impl Ingredient for Value {
    /// Get all dependencies of this ingredient
    fn get_deps(&self, value: FieldValue, row: Row, circular: bool) -> std::vec::Vec<Dep> {
        vec![]
    }

    /// Let the ingredient determine the value of the field to store in a serialization
    fn serialize(&self, value: FieldValue, row: Row, books: &BookKeeper, circular: bool) -> Option<FieldValue> {
        Some(value)
    }

    /// Let the ingredient determine the value of the field to insert into the database when deserializing
    fn deserialize(&self, value: FieldValue, row: Row, books: &BookKeeper) -> Option<DeserializedValue> {
        Some(DeserializedValue::new(vec![], value))
    }

    /// Should return an array with fields required to be able to UPDATE a row
    fn get_required_extra_fields(&self) -> std::vec::Vec<std::string::String> {
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
        fn resolve_id(&self, etype: EntityType, id: Id, authoritative: bool) -> Option<Id> { unimplemented!() }
        fn reset(&mut self) { unimplemented!() }
    }

    #[test]
    fn it_gets_deps() {
        let v = Value::new();

        assert_eq!(0, v.get_deps(FieldValue::Null, std::collections::HashMap::new(), false).len());
        assert_eq!(0, v.get_deps(FieldValue::Int(123), std::collections::HashMap::new(), false).len());
        assert_eq!(0, v.get_deps(FieldValue::String(std::string::String::from("Foo")), std::collections::HashMap::new(), false).len());
    }

    #[test]
    fn it_serializes() {
        let v = Value::new();
        let b = BookKeeperMock::new();

        assert_eq!(Some(FieldValue::Int(123)), v.serialize(FieldValue::Int(123), std::collections::HashMap::new(), &b, false));
    }

    #[test]
    fn it_deserializes() {
        let v = Value::new();
        let b = BookKeeperMock::new();

        let o = v.deserialize(FieldValue::Int(123), std::collections::HashMap::new(), &b);

        assert!(o.is_some());
        let d = o.unwrap();

        assert_eq!(FieldValue::Int(123), d.value());
        assert_eq!(0, d.deps().len());
    }

    #[test]
    fn it_gets_required_extra_fields() {
        let v = Value::new();

        assert_eq!(0, v.get_required_extra_fields().len())
    }
}