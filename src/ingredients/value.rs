use ingredients::ingredient::*;
use contracts::*;
use book_keeper::*;
use std::vec::Vec;
use std::string::String;
use std::collections::HashMap;

pub struct Value {}

impl Value {
    pub fn new() -> Value { Value {} }
}

#[derive(Serialize, Deserialize)]
struct ValueConfig {}

impl Ingredient<ValueConfig> for Value {
    /// Get all dependencies of this ingredient
    fn get_deps(&self, value: FieldValue, row: Row, circular: bool) -> Vec<Dep> {
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
    fn get_required_extra_fields(&self) -> Vec<String> {
        vec![]
    }

    /// Turn the ingredient into a config
    fn to_config(&self) -> IngredientConfig<ValueConfig> {
        IngredientConfig {
            type_: "VALUE",
            config: ValueConfig {}
        }
    }

    /// Create the ingredient from a config
    fn from_config(config: IngredientConfig<ValueConfig>) -> Self { Value {} }
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

        assert_eq!(0, v.get_deps(FieldValue::Null, HashMap::new(), false).len());
        assert_eq!(0, v.get_deps(FieldValue::Int(123), HashMap::new(), false).len());
        assert_eq!(0, v.get_deps(FieldValue::String(String::from("Foo")), HashMap::new(), false).len());
    }

    #[test]
    fn it_serializes() {
        let v = Value::new();
        let b = BookKeeperMock::new();

        assert_eq!(Some(FieldValue::Int(123)), v.serialize(FieldValue::Int(123), HashMap::new(), &b, false));
    }

    #[test]
    fn it_deserializes() {
        let v = Value::new();
        let b = BookKeeperMock::new();

        let o = v.deserialize(FieldValue::Int(123), HashMap::new(), &b);

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

    #[test]
    fn it_creates_a_config() {
        let v = Value::new();

        let config = v.to_config();

        assert_eq!("VALUE", config.type_);
    }

    #[test]
    fn it_creates_from_config() {
        let v = Value::from_config(IngredientConfig {
            type_: "VALUE",
            config: ValueConfig {}
        });
    }
}