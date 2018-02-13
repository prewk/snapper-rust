use ingredients::ingredient::*;
use contracts::*;
use book_keeper::*;
use std::vec::Vec;
use std::string::String;
use std::collections::HashMap;
use ingredients::raw::Raw;
use ingredients::reference::Reference;
use ingredients::value::Value;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CircularIngredient {
    Value(Value),
    Raw(Raw),
    Ref(Reference),
}

#[derive(Debug, Serialize, Deserialize)]
struct CircularConfig {
    ingredient: CircularIngredient,
    fallback: CircularIngredient,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Circular {
    #[serde(rename="type")]
    type_: String,
    config: CircularConfig,
}

impl Circular {
    pub fn new(ingredient: CircularIngredient, fallback: CircularIngredient) -> Circular {
        Circular {
            type_: "CIRCULAR".to_string(),
            config: CircularConfig {
                ingredient,
                fallback,
            }
        }
    }
}

impl Ingredient for Circular {
    /// Get all dependencies of this ingredient
    fn get_deps(&self, value: FieldValue, row: Row, circular: bool) -> Vec<Dep> {
        match circular {
            true => match &self.config.ingredient {
                &CircularIngredient::Value(ref v) => v.get_deps(value, row, false),
                &CircularIngredient::Raw(ref r) => r.get_deps(value, row, false),
                &CircularIngredient::Ref(ref r) => r.get_deps(value, row, false),
            },
            false => match &self.config.fallback {
                &CircularIngredient::Value(ref v) => v.get_deps(value, row, false),
                &CircularIngredient::Raw(ref r) => r.get_deps(value, row, false),
                &CircularIngredient::Ref(ref r) => r.get_deps(value, row, false),
            },
        }
    }

    /// Let the ingredient determine the value of the field to store in a serialization
    fn snapper_serialize(&self, value: FieldValue, row: Row, books: &BookKeeper, circular: bool) -> Option<FieldValue> {
        match circular {
            true => match &self.config.ingredient {
                &CircularIngredient::Value(ref v) => v.snapper_serialize(value, row, books, false),
                &CircularIngredient::Raw(ref r) => r.snapper_serialize(value, row, books, false),
                &CircularIngredient::Ref(ref r) => r.snapper_serialize(value, row, books, false),
            },
            false => match &self.config.fallback {
                &CircularIngredient::Value(ref v) => v.snapper_serialize(value, row, books, false),
                &CircularIngredient::Raw(ref r) => r.snapper_serialize(value, row, books, false),
                &CircularIngredient::Ref(ref r) => r.snapper_serialize(value, row, books, false),
            },
        }
    }

    /// Let the ingredient determine the value of the field to insert into the database when deserializing
    fn snapper_deserialize(&self, value: FieldValue, row: Row, books: &BookKeeper) -> Option<DeserializedValue> {
        match &self.config.ingredient {
            &CircularIngredient::Value(ref v) => v.snapper_deserialize(value, row, books),
            &CircularIngredient::Raw(ref r) => r.snapper_deserialize(value, row, books),
            &CircularIngredient::Ref(ref r) => r.snapper_deserialize(value, row, books),
        }
    }

    /// Should return an array with fields required to be able to UPDATE a row
    fn get_required_extra_fields(&self) -> Vec<String> {
        match &self.config.ingredient {
            &CircularIngredient::Value(ref v) => v.get_required_extra_fields(),
            &CircularIngredient::Raw(ref r) => r.get_required_extra_fields(),
            &CircularIngredient::Ref(ref r) => r.get_required_extra_fields(),
        }
    }
}