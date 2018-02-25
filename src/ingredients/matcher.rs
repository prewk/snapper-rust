use ingredients::ingredient::*;
use contracts::*;
use book_keeper::*;
use std::vec::Vec;
use std::string::String;
use std::collections::HashMap;
use ingredients::raw::Raw;
use ingredients::reference::Reference;
use ingredients::value::Value;
use ingredients::morph::Morph;
use regex::Regex;
use tools::field_value_to_string;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MatchIngredient {
    Value(Value),
    Raw(Raw),
    Ref(Reference),
    Morph(Morph),
}

#[derive(Debug, Serialize, Deserialize)]
struct MatchMapper {
    field: String,
    on: HashMap<String, MatchIngredient>,
    patterns: HashMap<String, MatchIngredient>,
    default: Option<MatchIngredient>,
}

impl MatchMapper {
    /// Perform a match and find the correct ingredient
    fn get_matched_ingredient(&self, row: &Row) -> Option<&MatchIngredient> {
        row.get(&self.field)
            .and_then(|val| {
                let string_val = field_value_to_string(&val);

                // Check on
                if self.on.contains_key(&string_val) {
                    return Some(self.on.get(&string_val).unwrap());
                }

                // Check patterns
                for (pattern, ingredient) in &self.patterns {
                    let m = Regex::new(pattern)
                        .map(|re| {
                            re.is_match(&string_val[..])
                        })
                        .unwrap_or(false);

                    if m {
                        return Some(&ingredient);
                    }
                }

                // Default?
                self.default.as_ref()
            })
    }
}

impl MatchMapper {
    /// Get all dependencies of this ingredient
    fn get_deps(&self, value: &FieldValue, row: &Row, circular: bool) -> Vec<Dep> {
        self.get_matched_ingredient(&row)
            .map(|ingredient| {
                match ingredient {
                    &MatchIngredient::Value(ref v) => v.get_deps(&value, &row, circular),
                    &MatchIngredient::Raw(ref r) => r.get_deps(&value, &row, circular),
                    &MatchIngredient::Ref(ref r) => r.get_deps(&value, &row, circular),
                    &MatchIngredient::Morph(ref m) => m.get_deps(&value, &row, circular),
                }
            })
            .unwrap_or(vec![])
    }

    /// Let the ingredient determine the value of the field to store in a serialization
    fn snapper_serialize(&self, value: &FieldValue, row: &Row, books: &BookKeeper, circular: bool) -> Option<FieldValue> {
        self.get_matched_ingredient(&row)
            .map(|ingredient| {
                match ingredient {
                    &MatchIngredient::Value(ref v) => v.snapper_serialize(&value, &row, books, circular),
                    &MatchIngredient::Raw(ref r) => r.snapper_serialize(&value, &row, books, circular),
                    &MatchIngredient::Ref(ref r) => r.snapper_serialize(&value, &row, books, circular),
                    &MatchIngredient::Morph(ref m) => m.snapper_serialize(&value, &row, books, circular),
                }
            })
            .unwrap_or(None)
    }

    /// Let the ingredient determine the value of the field to insert into the database when deserializing
    fn snapper_deserialize(&self, value: &FieldValue, row: &Row, books: &BookKeeper) -> Option<DeserializedValue> {
        self.get_matched_ingredient(&row)
            .map(|ingredient| {
                match ingredient {
                    &MatchIngredient::Value(ref v) => v.snapper_deserialize(&value, &row, books),
                    &MatchIngredient::Raw(ref r) => r.snapper_deserialize(&value, &row, books),
                    &MatchIngredient::Ref(ref r) => r.snapper_deserialize(&value, &row, books),
                    &MatchIngredient::Morph(ref m) => m.snapper_deserialize(&value, &row, books),
                }
            })
            .unwrap_or(None)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct MatchConfig {
    field: String,
    matcher: MatchMapper,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Matcher {
    #[serde(rename="type")]
    type_: String,
    config: MatchConfig,
}

impl Matcher {
    pub fn new(field: String, on: HashMap<String, MatchIngredient>, patterns: HashMap<String, MatchIngredient>, default: Option<MatchIngredient>) -> Matcher {
        Matcher {
            type_: "MATCH".to_string(),
            config: MatchConfig {
                field: field.clone(),
                matcher: MatchMapper {
                    field,
                    on,
                    patterns,
                    default,
                }
            }
        }
    }
}

impl Ingredient for Matcher {
    /// Get all dependencies of this ingredient
    fn get_deps(&self, value: &FieldValue, row: &Row, circular: bool) -> Vec<Dep> {
        self.config.matcher.get_deps(&value, &row, circular)
    }

    /// Let the ingredient determine the value of the field to store in a serialization
    fn snapper_serialize(&self, value: &FieldValue, row: &Row, books: &BookKeeper, circular: bool) -> Option<FieldValue> {
        self.config.matcher.snapper_serialize(&value, &row, books, circular)
    }

    /// Let the ingredient determine the value of the field to insert into the database when deserializing
    fn snapper_deserialize(&self, value: &FieldValue, row: &Row, books: &BookKeeper) -> Option<DeserializedValue> {
        self.config.matcher.snapper_deserialize(&value, &row, books)
    }

    /// Should return an array with fields required to be able to UPDATE a row
    fn get_required_extra_fields(&self) -> Vec<String> {
        vec![self.config.field.clone()]
    }
}
