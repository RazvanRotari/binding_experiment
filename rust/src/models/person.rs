use serde::Deserialize;

use bind_gen::*;
use container::MapAccess;
use error::BindError;
use traits::{Deserializable, Deserializer, Visitor};

#[derive(Deserializable, Deserialize, Debug)]
pub struct Player {
    pub id: u64,
    pub name: String,
    pub nickname: String,
    pub age: u8,
    pub credits: f32,
    pub has_companion: bool,
    pub game_score: i32,
}

impl Player {
    pub fn new(id: u64, name: String, nickname: String, age: u8, credits: f32, has_companion: bool, game_score: i32) -> Self {
        Player { id, name, nickname, age, credits, has_companion, game_score }
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
            self.nickname == other.nickname &&
            self.age == other.age &&
            self.credits == other.credits &&
            self.has_companion == other.has_companion &&
            self.game_score == other.game_score
    }
}