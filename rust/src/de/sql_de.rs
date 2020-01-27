use std::str;

use rusqlite::Row;
use rusqlite::types::{FromSql, FromSqlError, ValueRef};

use container::map_iter::Map;
use traits::{Deserializer, Visitor};
use types::{Numeric, Value};

impl<'de> Deserializer for &Row<'de> {
    fn deserialize<V: Visitor>(self, visitor: V) -> Result<V::Value, V::Error> {
        let mut map: Map<String, Value> = Map::new();
        match self.column_count() {
            0 => visitor.visit_none(),
            1 => {
                match self.get::<usize, Value>(0) {
                    Ok(value) => value.deserialize(visitor),
                    _ => unimplemented!()
                }
            }
            n => {
                for i in 0..n {
                    let string_result = self.column_name(i)
                        .map(String::from);
                    let value_result = self.get(i);

                    match (string_result, value_result) {
                        (Ok(string), Ok(value)) => {
                            map.insert(string, value);
                        }
                        (Err(_), _) |
                        (_, Err(_)) => {
                            unimplemented!()
                        }
                    }
                }
                visitor.visit_map(map)
            }
        }
    }
}

impl FromSql for Value {
    fn column_result(value: ValueRef<'_>) -> Result<Self, FromSqlError> {
        match value {
            ValueRef::Null => Ok(Value::Null),
            ValueRef::Integer(value) => Ok(Value::Number(Numeric::I64(value))),
            ValueRef::Real(value) => Ok(Value::Number(Numeric::F64(value))),
            ValueRef::Blob(blob) |
            ValueRef::Text(blob) => {
                str::from_utf8(blob)
                    .map(String::from)
                    .map(Value::String)
                    .map_err(|e| FromSqlError::Other(Box::new(e)))
            }
        }
    }
}

