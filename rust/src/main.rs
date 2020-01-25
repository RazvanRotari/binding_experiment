extern crate bind_lib;
extern crate json;


use bind_lib::Deserialization;

#[derive(Debug, Deserialization)]
struct Simple {
    a: i32,
    b: String,
    c: f64,
    d: SimpleChild,
}

#[derive(Debug, Deserialization)]
struct SimpleChild {
    e: i32
}


// trait Extractor<T> {
//     fn convert(real_parser: &T) -> Self;
// }

// impl Extractor<json::JsonValue> for i32 {
//     fn convert(real_parser: &T) -> Self {
//         real_parser.as_i32().expect("Must be i32")  
//     }
// }

// impl Extractor<json::JsonValue> for str {
//     fn convert(real_parser: &T) -> Self {
//         real_parser.as_str().expect("Must be str").as_owned()
//     }
// }

// impl Extractor<json::JsonValue> for f64 {
//     fn convert(real_parser: &T) -> Self {
//         real_parser.as_f64().expect("Must be f64")  
//     }
// }


pub trait ParserProxy : Sized{
    fn as_i32(&self, key: &str) -> i32;
    fn as_String(&self, key: &str) -> String;
    fn as_f64(&self, key: &str) -> f64;
    fn as_object<T:Deserialization>(&self, key:&str) -> T;
}

struct Parser<T> {
    real_parser: T,
}

impl ParserProxy for Parser<json::JsonValue> {
    fn as_i32(&self, key: &str) -> i32 {
        self.real_parser[key].as_i32().unwrap()
    }
    fn as_String(&self, key: &str) -> String {
        self.real_parser[key].as_str().unwrap().to_owned()
    }
    fn as_f64(&self, key: &str) -> f64 {
        self.real_parser[key].as_f64().unwrap()
    }
    fn as_object<T:Deserialization>(&self, key:&str) -> T {
        let parser = Parser::<json::JsonValue> {
            real_parser: self.real_parser[key].clone(),
        };
        T::construct(parser)
    }
}
pub trait Deserialization {
    fn construct(j: impl ParserProxy) -> Self;
}


pub fn read_json<T:Deserialization>(jsonString: &str) -> T {
    let parser = Parser::<json::JsonValue> {
        real_parser: json::parse(jsonString).unwrap(),
    };
    T::construct(parser)
}


//////Construct this function with a macro
//impl Deserialization for Simple {
//    fn construct(j: impl ParserProxy) -> Self {
//        Simple {
//            a: j.as_i32("a"),
//            b: j.as_str("b"),
//            c: j.as_f64("c"),
//            d: j.as_object("d")
//        }
//    }
//}
/////
//////Construct this function with a macro
//impl Deserialization for SimpleChild {
//    fn construct(j: impl ParserProxy) -> Self {
//        SimpleChild {
//            e: j.as_i32("e"),
//        }
//    }
//}


//fn construct_simple(j: dyn Deserialization) -> Simple {
//   Simple{a:j["a"], b:j["b"], c:j["c"]}
//}

fn main() {
    let simple:Simple = read_json(r#"{"a":1, "b":"2", "c":3.0, "d":{"e": 5}}"#);
    println!("{:?}", simple);
}
