use container::{MapAccess, SeqAccess};
use error::BindError;

pub trait Deserializer: Sized {
    fn deserialize<V: Visitor>(self, visitor: V) -> Result<V::Value, V::Error>;
}

pub trait Deserializable: Sized {
    fn unmarshal<D: Deserializer>(deserializer: D) -> Result<Self, BindError>;
}

#[allow(unused_variables)]
pub trait Visitor: Sized {
    type Value;
    type Error;

    fn visit<V: Visitor>(self, v: V) -> Result<Self::Value, Self::Error> {
        unimplemented!()
    }

    fn visit_bool(self, v: bool) -> Result<Self::Value, Self::Error> {
        unimplemented!()
    }

    fn visit_i8(self, v: i8) -> Result<Self::Value, Self::Error> {
        unimplemented!()
    }

    fn visit_i16(self, v: i16) -> Result<Self::Value, Self::Error> {
        unimplemented!()
    }

    fn visit_i32(self, v: i32) -> Result<Self::Value, Self::Error> {
        unimplemented!()
    }

    fn visit_i64(self, v: i64) -> Result<Self::Value, Self::Error> {
        unimplemented!()
    }

    fn visit_isize(self, v: isize) -> Result<Self::Value, Self::Error> {
        unimplemented!()
    }

    fn visit_usize(self, v: usize) -> Result<Self::Value, Self::Error> {
        unimplemented!()
    }

    fn visit_u8(self, v: u8) -> Result<Self::Value, Self::Error> {
        unimplemented!()
    }

    fn visit_u16(self, v: u16) -> Result<Self::Value, Self::Error> {
        unimplemented!()
    }

    fn visit_u32(self, v: u32) -> Result<Self::Value, Self::Error> {
        unimplemented!()
    }

    fn visit_u64(self, v: u64) -> Result<Self::Value, Self::Error> {
        unimplemented!()
    }

    fn visit_f32(self, v: f32) -> Result<Self::Value, Self::Error> {
        unimplemented!()
    }

    fn visit_f64(self, v: f64) -> Result<Self::Value, Self::Error> {
        unimplemented!()
    }

    fn visit_char(self, v: char) -> Result<Self::Value, Self::Error> {
        unimplemented!()
    }

    fn visit_string(self, v: String) -> Result<Self::Value, Self::Error> {
        unimplemented!()
    }

    //TODO
    fn visit_unit(self) -> Result<Self::Value, Self::Error> {
        unimplemented!()
    }

    fn visit_seq<A: SeqAccess>(self, seq: A) -> Result<Self::Value, Self::Error> {
        unimplemented!()
    }

    fn visit_map<A: MapAccess>(self, map: A) -> Result<Self::Value, Self::Error> {
        unimplemented!()
    }

    //TODO
    fn visit_enum<V: Visitor>(self, data: V) -> Result<Self::Value, Self::Error> {
        unimplemented!()
    }

    //TODO
    fn visit_newtype_struct<D: Deserializer>(self, deserializer: D) -> Result<Self::Value, Self::Error> {
        unimplemented!()
    }

    fn visit_none(self) -> Result<Self::Value, Self::Error> {
        unimplemented!()
    }

    fn visit_some<D: Deserializer>(self, deserializer: D) -> Result<Self::Value, Self::Error> {
        unimplemented!()
    }

    fn visit_str(self, v: &str) -> Result<Self::Value, Self::Error> {
        unimplemented!()
    }
}
