extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro::TokenTree;

#[proc_macro_derive(Deserializable)]
pub fn derive_answer_fn(item: TokenStream) -> TokenStream {
    let mut opt_struct_name: Option<String> = None;
    let mut struct_fields: Vec<String> = Vec::new();
    for x in item.into_iter() {
        match &x {
            TokenTree::Ident(ident) => {
                if opt_struct_name.is_none() {
                    let string = ident.to_string();
                    match string.as_str() {
                        "pub" | "struct" => None,
                        _ => opt_struct_name.replace(string)
                    };
                }
            }
            TokenTree::Group(group) => {
                let mut ignore_input = false;
                for line in group.stream().into_iter() {
                    match &line {
                        TokenTree::Ident(ident) => {
                            if !ignore_input {
                                let string = ident.to_string();
                                match string.as_str() {
                                    "pub" => (),
                                    value => {
                                        struct_fields.push(value.to_owned());
                                    }
                                }
                            }
                        }
                        TokenTree::Punct(ch) => {
                            ignore_input = match ch.as_char() {
                                ',' => false,
                                ':' => true,
                                _ => ignore_input
                            }
                        }
                        _ => ()
                    }
                }
            }
            _ => {}
        }
    };
    let name = opt_struct_name.expect("expected to contain struct name");
    let result = implement_visitor(&name, implement_body(&name, struct_fields));
    //println!("Parsing result {}", result);
    result.parse().unwrap()
}

fn implement_body(struct_name: &String, fields: Vec<String>) -> String {
    let mapping: String = fields.into_iter()
        .map(|field| {
            format!(r#"
                    {0}: map.get_value("{0}")?.expect("A value for {0}"),
                    "#, field)
        }).collect();

    format!(r#"Ok({0} {{
            {1}
            }})"#, struct_name, mapping)
}

fn implement_visitor(struct_name: &String, body: String) -> String {
    let visitor_name = format!("{}Visitor", struct_name);
    format!(r#"
    impl Deserializable for {0} {{
           fn unmarshal<D: Deserializer>(deserializer: D) -> Result<Self, BindError> {{
            struct {1};

            impl Visitor for {1} {{
                type Value = {0};
                type Error = BindError;

                fn visit_map<A: MapAccess>(self, mut map: A) -> Result<Self::Value, Self::Error> {{
                    {2}
                }}
            }}
            deserializer.deserialize({1})
        }}
    }}
    "#, struct_name, visitor_name, body)
}