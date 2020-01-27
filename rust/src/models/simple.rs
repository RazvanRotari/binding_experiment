use serde::Deserialize;

use bind_gen::*;
use container::MapAccess;
use error::BindError;
use traits::{Deserializable, Deserializer, Visitor};

#[derive(Deserializable, Deserialize, Debug)]
pub struct Simple {
    pub a: i32,
    pub b: String,
    pub c: f64,
}