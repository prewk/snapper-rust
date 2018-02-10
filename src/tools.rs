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

    fn id_to_field_value_converts_uuid_to_string() {
        assert_eq!(FieldValue::String(String::from("Foo")), id_to_field_value(Id::Uuid(String::from("Foo"))));
    }
}