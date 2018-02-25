use ingredients::value::*;
use ingredients::raw::*;
use ingredients::reference::*;
use ingredients::circular::*;
use ingredients::morph::*;
use std::string::String;
use std::collections::HashMap;
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum PrimaryKey {
    Null,
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Ingredient {
    Value(Value),
    Raw(Raw),
    Ref(Reference),
    Circular(Circular),
    Morph(Morph),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    primary_key: PrimaryKey,
    ingredients: HashMap<String, Ingredient>
}

impl Recipe {
    fn primary_key(&self) -> &PrimaryKey {
        &self.primary_key
    }

    fn ingredient(&self, table: &String) -> Option<&Ingredient> {
        self.ingredients.get(table)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_reserialize() {
        let json = r#"{
            "primary_key": "id",
            "ingredients": {
                "name": { "type": "VALUE", "config": {} },
                "foo": { "type": "RAW", "config": {
                    "value": 123
                } },
                "foo_id": { "type": "REF", "config": {
                    "type": "foos",
                    "optional_values": []
                } },
                "bar_id": { "type": "CIRCULAR", "config": {
                    "ingredient": { "type": "REF", "config": { "type": "bars", "optional_values": [null] } },
                    "fallback": { "type": "RAW", "config": { "value": null } }
                } },
                "bazable_type": { "type": "VALUE", "config": {} },
                "bazable_id": { "type": "MORPH", "config": {
                    "field": "bazable_type",
                    "morph_mapper": { "morph_map": {
                        "FOO": "foos",
                        "BAR": "bars"
                    } },
                    "optional_values": []
                } }
            }
        }"#;

        let r: Recipe = serde_json::from_str(json).unwrap();

        let _back = serde_json::to_string(&r).unwrap();
    }
}