use std;
use contracts::*;
use book_keeper::*;

pub trait Ingredient {
    /// Get all dependencies of this ingredient
    fn get_deps(&self, value: FieldValue, row: Row, circular: bool) -> std::vec::Vec<Dep>;

    /// Let the ingredient determine the value of the field to store in a serialization
    fn serialize(&self, value: FieldValue, row: Row, books: &BookKeeper, circular: bool) -> Option<FieldValue>;

    /// Let the ingredient determine the value of the field to insert into the database when deserializing
    fn deserialize(&self, value: FieldValue, row: Row, books: &BookKeeper) -> Option<DeserializedValue>;

    /// Should return an array with fields required to be able to UPDATE a row
    fn get_required_extra_fields(&self) -> std::vec::Vec<std::string::String>;
}