use std::slice::Iter;
use std::vec::IntoIter;

use json::JsonValue;
use json::number::Number;
use json::object::Object;

use container::{MapAccess, SeqAccess};
use error::BindError;
use traits::{Deserializable, Deserializer, Visitor};

impl Deserializer for JsonValue {
    fn deserialize<V: Visitor>(self, visitor: V) -> Result<V::Value, V::Error> {
        match self {
            JsonValue::Null => visitor.visit_none(),
            JsonValue::Boolean(a_bool) => visitor.visit_bool(a_bool),
            JsonValue::Short(short) => visitor.visit_str(short.as_ref()),
            JsonValue::Number(num) => visit_number(num, visitor),
            JsonValue::String(string) => visitor.visit_string(string),
            JsonValue::Array(arr) => visitor.visit_seq(arr.into_iter()),
            JsonValue::Object(obj) => visitor.visit_map(obj),
        }
    }
}

impl Deserializer for &JsonValue {
    fn deserialize<V: Visitor>(self, visitor: V) -> Result<V::Value, V::Error> {
        match self {
            JsonValue::Null => visitor.visit_none(),
            JsonValue::Boolean(a_bool) => visitor.visit_bool(*a_bool),
            JsonValue::Short(short) => visitor.visit_str(short.as_ref()),
            JsonValue::Number(num) => visit_number(*num, visitor),
            JsonValue::String(string) => visitor.visit_str(string.as_str()),
            JsonValue::Array(arr) => visitor.visit_seq(arr.into_iter()),
            JsonValue::Object(obj) => visitor.visit_map(obj),
        }
    }
}

fn visit_number<V: Visitor>(number: Number, visitor: V) -> Result<V::Value, V::Error> {
    let (positive, _, exponent) = number.as_parts();
    match (positive, exponent >= 0) {
        (true, true) => visitor.visit_u64(number.into()),
        (false, true) => visitor.visit_i64(number.into()),
        (_, false) => visitor.visit_f64(number.into())
    }
}

impl MapAccess for Object {
    fn get_keys<K: Deserializable>(&mut self) -> Vec<Result<K, BindError>> {
        unimplemented!()
    }

    fn get_value<V: Deserializable>(&mut self, key: &str) -> Result<Option<V>, BindError> {
        let value: Option<&JsonValue> = self.get(key);
        match value {
            Some(json) => V::unmarshal(json)
                .map(Option::Some),
            None => Ok(None)
        }
    }
}


impl MapAccess for &Object {
    fn get_keys<K: Deserializable>(&mut self) -> Vec<Result<K, BindError>> {
        unimplemented!()
    }

    fn get_value<V: Deserializable>(&mut self, key: &str) -> Result<Option<V>, BindError> {
        let value: Option<&JsonValue> = self.get(key);
        match value {
            Some(json) => V::unmarshal(json)
                .map(Option::Some),
            None => Ok(None)
        }
    }
}

impl SeqAccess for Iter<'_, JsonValue> {
    fn next_element<T: Deserializable>(&mut self) -> Result<Option<T>, BindError> {
        match self.next() {
            Some(json) => T::unmarshal(json)
                .map(Option::Some),
            None => Ok(None)
        }
    }
}

impl SeqAccess for IntoIter<JsonValue> {
    fn next_element<T: Deserializable>(&mut self) -> Result<Option<T>, BindError> {
        match self.next() {
            Some(json) => T::unmarshal(json)
                .map(Option::Some),
            None => Ok(None)
        }
    }
}