use container::map_iter::Map;
use error::BindError;
use traits::Deserializable;
use types::{Numeric, Value};
use models::person::Player;

#[test]
fn should_read_person_with_matching_types() {
    let mut field_map: Map<String, Value> = Map::new();

    field_map.insert(String::from("id"), Value::Number(Numeric::U8(1)));
    field_map.insert(String::from("name"), Value::String(String::from("Randy")));
    field_map.insert(String::from("nickname"), Value::String(String::from("Brown")));
    field_map.insert(String::from("age"), Value::Number(Numeric::U64(30)));
    field_map.insert(String::from("credits"), Value::Number(Numeric::F64(-20f64)));
    field_map.insert(String::from("has_companion"), Value::Bool(true));
    field_map.insert(String::from("game_score"), Value::Number(Numeric::I8(100)));
    let value = Value::Object(field_map);
    let option = Player::unmarshal(value);

    let person: Player = option.unwrap();

    assert_eq!(person.has_companion, true);
    assert_eq!(person.age, 30);
    assert_eq!(person.name, "Randy");
}

#[test]
fn should_read_person_from_string() {
    let mut field_map: Map<String, Value> = Map::new();

    field_map.insert(String::from("id"), Value::String(String::from("1")));
    field_map.insert(String::from("name"), Value::String(String::from("Randy")));
    field_map.insert(String::from("nickname"), Value::String(String::from("Brown")));
    field_map.insert(String::from("age"), Value::String(String::from("30")));
    field_map.insert(String::from("credits"), Value::String(String::from("-16")));
    field_map.insert(String::from("has_companion"), Value::String(String::from("true")));
    field_map.insert(String::from("game_score"), Value::String(String::from("100")));

    let value = Value::Object(field_map);
    let option = Player::unmarshal(value);

    let person: Player = option.ok().unwrap();

    assert_eq!(person.has_companion, true);
    assert_eq!(person.age, 30);
    assert_eq!(person.name, "Randy");
}


#[test]
fn should_panic_when_reading_person() {
    let mut field_map: Map<String, Value> = Map::new();

    field_map.insert("name".to_owned(), Value::String("harry".to_owned()));
    field_map.insert("age".to_owned(), Value::String("22a".to_owned()));
    field_map.insert("has_companion".to_owned(), Value::String("true".to_owned()));

    let value = Value::Object(field_map);
    let option = Player::unmarshal(value);

    assert_eq!(option.is_err(), true);

    let error: BindError = option.err().unwrap();

    assert_eq!(error.get_message(), "Could not find id");
}
