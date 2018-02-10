use std::string::String;
use std::vec::Vec;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum FieldValue {
    Null,
    Int(i64),
    String(String),
}

pub type Row = HashMap<String, FieldValue>;

#[derive(Debug, PartialEq, Clone)]
pub enum Id {
    Int(u64),
    Uuid(String),
}

pub type EntityType = String;

pub type Dep = (EntityType, Id);

#[derive(Debug)]
pub struct DeserializedValue {
    deps: Vec<Dep>,
    value: FieldValue,
}

impl DeserializedValue {
    pub fn new(deps: Vec<Dep>, value: FieldValue) -> DeserializedValue {
        DeserializedValue {
            deps,
            value
        }
    }

    pub fn deps(&self) -> Vec<Dep> {
        self.deps.clone()
    }

    pub fn value(&self) -> FieldValue { self.value.clone() }
}
