use serde::Deserialize;

use bind_gen::*;
use container::MapAccess;
use error::BindError;
use traits::{Deserializable, Deserializer, Visitor};

#[derive(Deserializable, Deserialize, Debug)]
pub struct Menu {
    pub restaurant: String,
    pub items: Vec<MenuItem>,
}

#[derive(Deserializable, Deserialize, Debug)]
pub struct MenuItem {
    pub name: String,
    pub price: f32,
    pub vegetarian: bool,
    pub ingredients: Box<[String]>,
}