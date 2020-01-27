use error::BindError;
use traits::{Deserializable, Deserializer, Visitor};
use from_primitive::{FromIntPrimitive, FromFloatPrimitive};

struct CharVisitor;

struct StringVisitor;

impl Visitor for StringVisitor {
    type Value = String;
    type Error = BindError;

    fn visit_string(self, v: String) -> Result<Self::Value, Self::Error> {
        Ok(v)
    }

    fn visit_str(self, v: &str) -> Result<Self::Value, Self::Error> {
        Ok(v.to_owned())
    }
}

impl Visitor for CharVisitor {
    type Value = char;
    type Error = BindError;

    fn visit_char(self, v: char) -> Result<Self::Value, Self::Error> {
        Ok(v)
    }
}

impl Deserializable for String {
    fn unmarshal<D: Deserializer>(deserializer: D) -> Result<Self, BindError> {
        deserializer.deserialize(StringVisitor)
    }
}

/**
    TODO: requires a serious overhaul
*/
macro_rules! implement_methods {
    ($ty:ty, {$( $method:ident:$type:ty => $castor:ident ),*}) => {
        $(
        fn $method(self, v: $type) -> Result<Self::Value, Self::Error> {
            <$ty>::$castor(v).ok_or(BindError::from_string(
                format!("Could not convert {} from {}", stringify!($ty), stringify!($type))
            ))
        }
        )*
    }
}

macro_rules! implement_from_casts {
    ($visitor_name:ident, $ty:ty as base_type, {$( $method:ident:$type:ty => $castor:ident ),*}) => {
        struct $visitor_name;

        impl Visitor for $visitor_name {
            type Value = $ty;
            type Error = BindError;

            $(
            fn $method(self, v: $type) -> Result<Self::Value, Self::Error> {
                <$ty>::$castor(v).ok_or(BindError::from_string(
                    format!("Could not convert {} from {}", stringify!($ty), stringify!($type))
                ))
            }
            )*

             fn visit_string(self, v: String) -> Result<Self::Value, Self::Error> {
                v.parse::<$ty>()
                    .map_err(|_e| BindError::from_string(format!("Could not parse string to {}", stringify!($ty))))
            }

            fn visit_str(self, v: &str) -> Result<Self::Value, Self::Error> {
                v.parse::<$ty>()
                    .map_err(|_e| BindError::from_string(format!("Could not parse string to {}", stringify!($ty))))
            }
        }

        impl Deserializable for $ty {
            fn unmarshal<D: Deserializer>(deserializer: D) -> Result<Self, BindError> {
                deserializer.deserialize($visitor_name)
            }
        }
    }
}

macro_rules! implement_one_to_one {
    ($visitor_name:ident, $ty:ty as base_type, {$( $method:ident:$type:ty ),*}) => {

        struct $visitor_name;

        impl Visitor for $visitor_name {
            type Value = $ty;
            type Error = BindError;

            $(
            fn $method(self, v: $type) -> Result<Self::Value, Self::Error> {
                Ok(<$ty>::from(v))
            }
            )*

            fn visit_string(self, v: String) -> Result<Self::Value, Self::Error> {
                v.parse::<$ty>()
                    .map_err(|_e| BindError::from_string(format!("Could not parse string to {}", stringify!($ty))))
            }

            fn visit_str(self, v: &str) -> Result<Self::Value, Self::Error> {
                v.parse::<$ty>()
                    .map_err(|_e| BindError::from_string(format!("Could not parse string to {}", stringify!($ty))))
            }

            implement_methods!($ty, {
                visit_i8: i8 => from_i8,
                visit_i16: i16 => from_i16,
                visit_i32: i32 => from_i32,
                visit_i64: i64 => from_i64,
                visit_u8: u8 => from_u8,
                visit_u16: u16 => from_u16,
                visit_u32: u32 => from_u32,
                visit_u64: u64 => from_u64
            });
        }

        impl Deserializable for $ty {
            fn unmarshal<D: Deserializer>(deserializer: D) -> Result<Self, BindError> {
                deserializer.deserialize($visitor_name)
            }
        }
    }
}

implement_from_casts!(I8Visitor, i8 as base_type, {
    visit_i8: i8 => from_i8,
    visit_i16: i16 => from_i16,
    visit_i32: i32 => from_i32,
    visit_i64: i64 => from_i64,
    visit_u8: u8 => from_u8,
    visit_u16: u16 => from_u16,
    visit_u32: u32 => from_u32,
    visit_u64: u64 => from_u64
});
implement_from_casts!(I16Visitor, i16 as base_type, {
    visit_i8: i8 => from_i8,
    visit_i16: i16 => from_i16,
    visit_i32: i32 => from_i32,
    visit_i64: i64 => from_i64,
    visit_u8: u8 => from_u8,
    visit_u16: u16 => from_u16,
    visit_u32: u32 => from_u32,
    visit_u64: u64 => from_u64
});
implement_from_casts!(I32Visitor, i32 as base_type, {
    visit_i8: i8 => from_i8,
    visit_i16: i16 => from_i16,
    visit_i32: i32 => from_i32,
    visit_i64: i64 => from_i64,
    visit_u8: u8 => from_u8,
    visit_u16: u16 => from_u16,
    visit_u32: u32 => from_u32,
    visit_u64: u64 => from_u64
});
implement_from_casts!(I64Visitor, i64 as base_type, {
    visit_i8: i8 => from_i8,
    visit_i16: i16 => from_i16,
    visit_i32: i32 => from_i32,
    visit_i64: i64 => from_i64,
    visit_u8: u8 => from_u8,
    visit_u16: u16 => from_u16,
    visit_u32: u32 => from_u32,
    visit_u64: u64 => from_u64
});

implement_from_casts!(U8Visitor, u8 as base_type, {
    visit_i8: i8 => from_i8,
    visit_i16: i16 => from_i16,
    visit_i32: i32 => from_i32,
    visit_i64: i64 => from_i64,
    visit_u8: u8 => from_u8,
    visit_u16: u16 => from_u16,
    visit_u32: u32 => from_u32,
    visit_u64: u64 => from_u64
});
implement_from_casts!(U16Visitor, u16 as base_type, {
    visit_i8: i8 => from_i8,
    visit_i16: i16 => from_i16,
    visit_i32: i32 => from_i32,
    visit_i64: i64 => from_i64,
    visit_u8: u8 => from_u8,
    visit_u16: u16 => from_u16,
    visit_u32: u32 => from_u32,
    visit_u64: u64 => from_u64
});
implement_from_casts!(U32Visitor, u32 as base_type, {
    visit_i8: i8 => from_i8,
    visit_i16: i16 => from_i16,
    visit_i32: i32 => from_i32,
    visit_i64: i64 => from_i64,
    visit_u8: u8 => from_u8,
    visit_u16: u16 => from_u16,
    visit_u32: u32 => from_u32,
    visit_u64: u64 => from_u64
});
implement_from_casts!(U64Visitor, u64 as base_type, {
    visit_i8: i8 => from_i8,
    visit_i16: i16 => from_i16,
    visit_i32: i32 => from_i32,
    visit_i64: i64 => from_i64,
    visit_u8: u8 => from_u8,
    visit_u16: u16 => from_u16,
    visit_u32: u32 => from_u32,
    visit_u64: u64 => from_u64
});

implement_from_casts!(F32Visitor, f32 as base_type, {
    visit_f32: f32 => from_f32,
    visit_f64: f64 => from_f64
});
implement_from_casts!(F64Visitor, f64 as base_type, {
    visit_f32: f32 => from_f32,
    visit_f64: f64 => from_f64
});

implement_one_to_one!(BoolVisitor, bool as base_type, {
    visit_bool: bool
});
implement_one_to_one!(ISizeVisitor, isize as base_type, {
    visit_isize: isize
});
implement_one_to_one!(USizeVisitor, usize as base_type, {
    visit_usize: usize
});