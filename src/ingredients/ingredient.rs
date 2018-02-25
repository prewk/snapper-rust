use contracts::*;
use book_keeper::*;
use std::vec::Vec;
use std::string::String;

pub trait Ingredient {
    /// Get all dependencies of this ingredient
    fn get_deps(&self, value: &FieldValue, row: &Row, circular: bool) -> Vec<Dep>;

    /// Let the ingredient determine the value of the field to store in a serialization
    fn snapper_serialize(&self, value: &FieldValue, row: &Row, books: &BookKeeper, circular: bool) -> Option<FieldValue>;

    /// Let the ingredient determine the value of the field to insert into the database when deserializing
    fn snapper_deserialize(&self, value: &FieldValue, row: &Row, books: &BookKeeper) -> Option<DeserializedValue>;

    /// Should return an array with fields required to be able to UPDATE a row
    fn get_required_extra_fields(&self) -> Vec<String>;
}

