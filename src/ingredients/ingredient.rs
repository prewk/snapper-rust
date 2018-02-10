extern crate serde;
extern crate serde_json;

use std;
use contracts::*;
use book_keeper::*;

#[derive(Serialize, Deserialize)]
pub struct IngredientConfig<C> {
    pub type_: &'static str,
    pub config: C,
}

pub trait Ingredient<C> {
    /// Get all dependencies of this ingredient
    fn get_deps(&self, value: FieldValue, row: Row, circular: bool) -> std::vec::Vec<Dep>;

    /// Let the ingredient determine the value of the field to store in a serialization
    fn serialize(&self, value: FieldValue, row: Row, books: &BookKeeper, circular: bool) -> Option<FieldValue>;

    /// Let the ingredient determine the value of the field to insert into the database when deserializing
    fn deserialize(&self, value: FieldValue, row: Row, books: &BookKeeper) -> Option<DeserializedValue>;

    /// Should return an array with fields required to be able to UPDATE a row
    fn get_required_extra_fields(&self) -> std::vec::Vec<std::string::String>;

    /// Turn the ingredient into a config
    fn to_config(&self) -> IngredientConfig<C>;

    /// Create the ingredient from a config
    fn from_config(config: IngredientConfig<C>) -> Self;
}