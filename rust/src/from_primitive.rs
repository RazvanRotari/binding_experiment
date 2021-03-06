/**
    See Source:
    https://doc.servo.org/serde/de/from_primitive/trait.FromIntPrimitive.html
*/
pub trait FromIntPrimitive: Sized {
    fn from_i8(n: i8) -> Option<Self>;
    fn from_i16(n: i16) -> Option<Self>;
    fn from_i32(n: i32) -> Option<Self>;
    fn from_i64(n: i64) -> Option<Self>;
    fn from_u8(n: u8) -> Option<Self>;
    fn from_u16(n: u16) -> Option<Self>;
    fn from_u32(n: u32) -> Option<Self>;
    fn from_u64(n: u64) -> Option<Self>;
}


pub trait FromFloatPrimitive: Sized {
    fn from_f32(n: f32) -> Option<Self>;
    fn from_f64(n: f64) -> Option<Self>;
}


macro_rules! float_to_float {
    ($dst:ident, $n:ident) => {
        if std::$dst::MIN as f64 <= $n as f64 && $n as f64 <= std::$dst::MAX as f64 {
            Some($n as $dst)
        } else {
            None
        }
    };
}

macro_rules! int_to_int {
    ($dst:ident, $n:ident) => {
        if std::$dst::MIN as i64 <= $n as i64 && $n as i64 <= std::$dst::MAX as i64 {
            Some($n as $dst)
        } else {
            None
        }
    };
}

macro_rules! int_to_uint {
    ($dst:ident, $n:ident) => {
        if 0 <= $n && $n as u64 <= std::$dst::MAX as u64 {
            Some($n as $dst)
        } else {
            None
        }
    };
}

macro_rules! uint_to {
    ($dst:ident, $n:ident) => {
        if $n as u64 <= $dst::max_value() as u64 {
            Some($n as $dst)
        } else {
            None
        }
    };
}

macro_rules! impl_from_primitive_for_int {
    ($t:ident) => {
        impl FromIntPrimitive for $t {
            #[inline]
            fn from_i8(n: i8) -> Option<Self> {
                int_to_int!($t, n)
            }
            #[inline]
            fn from_i16(n: i16) -> Option<Self> {
                int_to_int!($t, n)
            }
            #[inline]
            fn from_i32(n: i32) -> Option<Self> {
                int_to_int!($t, n)
            }
            #[inline]
            fn from_i64(n: i64) -> Option<Self> {
                int_to_int!($t, n)
            }
            #[inline]
            fn from_u8(n: u8) -> Option<Self> {
                uint_to!($t, n)
            }
            #[inline]
            fn from_u16(n: u16) -> Option<Self> {
                uint_to!($t, n)
            }
            #[inline]
            fn from_u32(n: u32) -> Option<Self> {
                uint_to!($t, n)
            }
            #[inline]
            fn from_u64(n: u64) -> Option<Self> {
                uint_to!($t, n)
            }
        }
    };
}

macro_rules! impl_from_primitive_for_uint {
    ($t:ident) => {
        impl FromIntPrimitive for $t {
            #[inline]
            fn from_i8(n: i8) -> Option<Self> {
                int_to_uint!($t, n)
            }
            #[inline]
            fn from_i16(n: i16) -> Option<Self> {
                int_to_uint!($t, n)
            }
            #[inline]
            fn from_i32(n: i32) -> Option<Self> {
                int_to_uint!($t, n)
            }
            #[inline]
            fn from_i64(n: i64) -> Option<Self> {
                int_to_uint!($t, n)
            }
            #[inline]
            fn from_u8(n: u8) -> Option<Self> {
                uint_to!($t, n)
            }
            #[inline]
            fn from_u16(n: u16) -> Option<Self> {
                uint_to!($t, n)
            }
            #[inline]
            fn from_u32(n: u32) -> Option<Self> {
                uint_to!($t, n)
            }
            #[inline]
            fn from_u64(n: u64) -> Option<Self> {
                uint_to!($t, n)
            }
        }
    };
}

macro_rules! impl_from_primitive_for_float {
    ($t:ident) => {
        impl FromIntPrimitive for $t {
            #[inline]
            fn from_i8(n: i8) -> Option<Self> {
                Some(n as Self)
            }
            #[inline]
            fn from_i16(n: i16) -> Option<Self> {
                Some(n as Self)
            }
            #[inline]
            fn from_i32(n: i32) -> Option<Self> {
                Some(n as Self)
            }
            #[inline]
            fn from_i64(n: i64) -> Option<Self> {
                Some(n as Self)
            }
            #[inline]
            fn from_u8(n: u8) -> Option<Self> {
                Some(n as Self)
            }
            #[inline]
            fn from_u16(n: u16) -> Option<Self> {
                Some(n as Self)
            }
            #[inline]
            fn from_u32(n: u32) -> Option<Self> {
                Some(n as Self)
            }
            #[inline]
            fn from_u64(n: u64) -> Option<Self> {
                Some(n as Self)
            }
        }

        impl FromFloatPrimitive for $t {
            fn from_f32(n: f32) -> Option<Self> {
                float_to_float!($t, n)
            }

            fn from_f64(n: f64) -> Option<Self> {
                float_to_float!($t, n)
            }
        }
    };
}

impl_from_primitive_for_int!(isize);
impl_from_primitive_for_int!(i8);
impl_from_primitive_for_int!(i16);
impl_from_primitive_for_int!(i32);
impl_from_primitive_for_int!(i64);
impl_from_primitive_for_uint!(usize);
impl_from_primitive_for_uint!(u8);
impl_from_primitive_for_uint!(u16);
impl_from_primitive_for_uint!(u32);
impl_from_primitive_for_uint!(u64);
impl_from_primitive_for_float!(f32);
impl_from_primitive_for_float!(f64);

macro_rules! bool_to_int {
    ($n:ident) => {
        match $n {
            0 => Some(false),
            1 => Some(true),
            _ => None
        }
    };
}

// NOTE: Debatable trait implementation since it may be a source of errors
impl FromIntPrimitive for bool {
    fn from_i8(n: i8) -> Option<Self> {
        bool_to_int!(n)
    }

    fn from_i16(n: i16) -> Option<Self> {
        bool_to_int!(n)
    }

    fn from_i32(n: i32) -> Option<Self> {
        bool_to_int!(n)
    }

    fn from_i64(n: i64) -> Option<Self> {
        bool_to_int!(n)
    }

    fn from_u8(n: u8) -> Option<Self> {
        bool_to_int!(n)
    }

    fn from_u16(n: u16) -> Option<Self> {
        bool_to_int!(n)
    }

    fn from_u32(n: u32) -> Option<Self> {
        bool_to_int!(n)
    }

    fn from_u64(n: u64) -> Option<Self> {
        bool_to_int!(n)
    }
}
