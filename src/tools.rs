extern crate serde_json;

use std::string::String;
use contracts::*;

pub fn field_value_to_id(val: FieldValue) -> Option<Id> {
    match val {
        FieldValue::Null => None,
        FieldValue::Int(v) => Some(Id::Int(v as u64)),
        FieldValue::String(s) => Some(Id::Uuid(s.clone()))
    }
}

pub fn id_to_field_value(id: Id) -> FieldValue {
    match (id) {
        Id::Int(v) => FieldValue::Int(v as i64),
        Id::Uuid(s) => FieldValue::String(s.clone())
    }
}

pub fn field_value_to_serde_value(val: &FieldValue) -> serde_json::Value {
    match val {
        &FieldValue::Null => serde_json::Value::Null,
        &FieldValue::Int(v) => serde_json::Value::from(v),
        &FieldValue::String(ref s) => serde_json::Value::String(s.clone()),
    }
}

pub fn serde_value_to_field_value(val: &serde_json::Value) -> FieldValue {
    match val {
        &serde_json::Value::Null => FieldValue::Null,
        &serde_json::Value::Number(ref v) => FieldValue::Int(v.as_i64().unwrap_or(0 as i64)),
        &serde_json::Value::String(ref s) => FieldValue::String(s.clone()),
        _ => FieldValue::Null,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_value_to_id_converts_null_to_none() {
       assert_eq!(None, field_value_to_id(FieldValue::Null));
    }

    #[test]
    fn field_value_to_id_converts_int_to_int() {
        assert_eq!(Some(Id::Int(123)), field_value_to_id(FieldValue::Int(123)));
    }

    #[test]
    fn field_value_to_id_converts_string_to_uuid() {
        assert_eq!(Some(Id::Uuid(String::from("Foo"))), field_value_to_id(FieldValue::String(String::from("Foo"))))
    }

    #[test]
    fn id_to_field_value_converts_int_to_int() {
        assert_eq!(FieldValue::Int(123), id_to_field_value(Id::Int(123)));
    }

    #[test]
    fn id_to_field_value_converts_uuid_to_string() {
        assert_eq!(FieldValue::String(String::from("Foo")), id_to_field_value(Id::Uuid(String::from("Foo"))));
    }

    #[test]
    fn field_value_to_serde_value_converts_null_to_null() {
        assert_eq!(serde_json::Value::Null, field_value_to_serde_value(&FieldValue::Null));
    }

    #[test]
    fn field_value_to_serde_value_converts_int_to_number() {
        assert_eq!(serde_json::Value::from(123), field_value_to_serde_value(&FieldValue::Int(123)));
    }

    #[test]
    fn field_value_to_serde_value_converts_string_to_string() {
        assert_eq!(serde_json::Value::String(String::from("Foo")), field_value_to_serde_value(&FieldValue::String(String::from("Foo"))));
    }

    #[test]
    fn serde_value_to_field_value_converts_null_to_null() {
        assert_eq!(FieldValue::Null, serde_value_to_field_value(&serde_json::Value::Null));
    }

    #[test]
    fn serde_value_to_field_value_converts_number_to_int() {
        assert_eq!(FieldValue::Int(123), serde_value_to_field_value(&serde_json::Value::from(123)));
    }

    #[test]
    fn serde_value_to_field_value_converts_string_to_string() {
        assert_eq!(FieldValue::String(String::from("Foo")), serde_value_to_field_value(&serde_json::Value::String(String::from("Foo"))));
    }
}