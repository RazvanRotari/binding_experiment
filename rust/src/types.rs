use container::map_iter::Map;
use traits::{Deserializer, Visitor};

impl Deserializer for String {
    fn deserialize<V: Visitor>(self, visitor: V) -> Result<V::Value, V::Error> {
        visitor.visit_string(self)
    }
}

#[derive(Debug)]
pub enum Value {
    Null,
    Bool(bool),
    Char(char),
    Number(Numeric),
    String(String),
    Array(Vec<Value>),
    Object(Map<String, Value>),
}

impl Deserializer for Value {
    fn deserialize<V: Visitor>(self, visitor: V) -> Result<V::Value, V::Error> {
        match self {
            Value::Null => visitor.visit_unit(),
            Value::Bool(abool) => visitor.visit_bool(abool),
            Value::Number(number) => number.deserialize(visitor),
            Value::Char(character) => visitor.visit_char(character),
            Value::String(string) => visitor.visit_string(string),
            Value::Array(arr) => visitor.visit_seq(arr.into_iter()),
            Value::Object(map) => visitor.visit_map(map),
        }
    }
}

impl Deserializer for &Value {
    fn deserialize<V: Visitor>(self, visitor: V) -> Result<V::Value, V::Error> {
        match self {
            Value::Null => visitor.visit_unit(),
            Value::Bool(abool) => visitor.visit_bool(*abool),
            Value::Number(number) => number.deserialize(visitor),
            Value::Char(character) => visitor.visit_char(*character),
            Value::String(string) => visitor.visit_str(string),
            Value::Array(arr) => {
                visitor.visit_seq(arr.into_iter())
            }
            Value::Object(map) => {
                visitor.visit_map(map)
            }
        }
    }
}

#[derive(Debug)]
pub enum Numeric {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    ISIZE(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    USIZE(usize),
    F32(f32),
    F64(f64),
}

impl Deserializer for Numeric {
    fn deserialize<V: Visitor>(self, visitor: V) -> Result<V::Value, V::Error> {
        match self {
            Numeric::I8(number) => visitor.visit_i8(number),
            Numeric::I16(number) => visitor.visit_i16(number),
            Numeric::I32(number) => visitor.visit_i32(number),
            Numeric::I64(number) => visitor.visit_i64(number),
            Numeric::ISIZE(number) => visitor.visit_isize(number),
            Numeric::U8(number) => visitor.visit_u8(number),
            Numeric::U16(number) => visitor.visit_u16(number),
            Numeric::U32(number) => visitor.visit_u32(number),
            Numeric::U64(number) => visitor.visit_u64(number),
            Numeric::USIZE(number) => visitor.visit_usize(number),
            Numeric::F32(number) => visitor.visit_f32(number),
            Numeric::F64(number) => visitor.visit_f64(number),
        }
    }
}


impl Deserializer for &Numeric {
    fn deserialize<V: Visitor>(self, visitor: V) -> Result<V::Value, V::Error> {
        match self {
            Numeric::I8(number) => visitor.visit_i8(*number),
            Numeric::I16(number) => visitor.visit_i16(*number),
            Numeric::I32(number) => visitor.visit_i32(*number),
            Numeric::I64(number) => visitor.visit_i64(*number),
            Numeric::ISIZE(number) => visitor.visit_isize(*number),
            Numeric::U8(number) => visitor.visit_u8(*number),
            Numeric::U16(number) => visitor.visit_u16(*number),
            Numeric::U32(number) => visitor.visit_u32(*number),
            Numeric::U64(number) => visitor.visit_u64(*number),
            Numeric::USIZE(number) => visitor.visit_usize(*number),
            Numeric::F32(number) => visitor.visit_f32(*number),
            Numeric::F64(number) => visitor.visit_f64(*number),
        }
    }
}
