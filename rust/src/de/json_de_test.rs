use models::menu::{Menu, MenuItem};
use traits::Deserializable;
use models::simple::Simple;

const JSON_BODY: &str = r#"
    {
        "restaurant": "Fast-Fast-Food",
        "items": [{
                "name": "Burger",
                "price": 22.0,
                "vegetarian": false,
                "ingredients": [
                    "meat", "garlic", "onion", "hamburger buns", "mayonnaise", "ketchup"
                ]
            },
            {
                "name": "Pineapple Pizza",
                "price": 32.0,
                "vegetarian": false,
                "ingredients": [
                ]
            }
        ]
    }
    "#;

#[test]
fn should_read_object() {
    let json = json::parse(JSON_BODY).unwrap();
    let menu = Menu::unmarshal(json).unwrap();

    assert_eq!(menu.restaurant, "Fast-Fast-Food");
    assert_eq!(menu.items.len(), 2);

    assert_menu_item(menu.items.get(0),
                     "Burger", 22f32, false, 6);

    assert_menu_item(menu.items.get(1),
                     "Pineapple Pizza", 32f32, false, 0);
}

fn assert_menu_item(option: Option<&MenuItem>, name: &str, price: f32, vegetarian: bool, ingredient_count: usize) {
    assert_eq!(option.is_some(), true);
    let item = option.unwrap();
    assert_eq!(item.name, String::from(name));
    assert_eq!(item.price, price);
    assert_eq!(item.vegetarian, vegetarian);
    assert_eq!(item.ingredients.len(), ingredient_count);
}

#[test]
fn should_deserialize_simple() {
    const JSON_BODY: &str = r#"
    {
        "a": 22,
        "b": "test",
        "c": 12.1
    }
    "#;

    let json = json::parse(JSON_BODY).unwrap();
    Simple::unmarshal(json).unwrap();
}