#[macro_use]
extern crate criterion;
extern crate json;
extern crate rs_bind;
extern crate serde;
extern crate serde_json;

use criterion::Criterion;

use rs_bind::models::menu::Menu;
use rs_bind::traits::Deserializable;

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

fn criterion_benchmark(criterion: &mut Criterion) {
    criterion
        .bench_function("[Menu] json parsing using `json`", |b| b.iter(|| {
            json::parse(JSON_BODY).unwrap();
        }));
    criterion
        .bench_function("[Menu] json parsing + deserialization", |b| b.iter(|| {
            let json = json::parse(JSON_BODY).unwrap();
            Menu::unmarshal(json).unwrap()
        }));
    criterion
        .bench_function("[Menu] Serde parsing + deserialization", |b| b.iter(|| {
            serde_json::from_str::<Menu>(JSON_BODY).unwrap()
        }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);