extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro::TokenTree::{Group, Ident};

#[proc_macro_derive(Deserialization)]
pub fn derive_answer_fn(item: TokenStream) -> TokenStream {
    let blessed_tokes = "f64 i32 String".to_string();
    let mut index = 0;
    let mut output: String = "".to_string();
    for x in item.into_iter() {
        match &x {
            Ident(ident) => {
                if index == 1 {
                    output = format!(
                        r#"impl Deserialization for {0} {{
                            fn construct(j: impl ParserProxy) -> Self {{
                            {0} {{
                        "#,
                        ident
                    );
                }
                index += 1;
            }
            Group(group) => {
                let mut inner_index = 0;
                let mut name: String = "".to_string();
                for line in group.stream().into_iter() {
                    if inner_index % 4 == 0 {
                        name = line.to_string();
                    }
                    if inner_index % 4 == 2 {
                        let typ = if blessed_tokes.contains(line.to_string().as_str()) {
                            line.to_string()
                        } else {
                            "object".to_string()
                        };
                        output = format!(
                            r#"{0}
                                         {1}: j.as_{2}("{1}"),
                                         "#,
                            output,
                            name,
                            typ
                        );
                    }
                    println!("Line {} {:?}", inner_index % 4, line);

                    inner_index += 1;
                }
            }
            _ => {}
        };
        println!("x {:?}", x);
    }
    output = format!("{} \n }} \n}} \n}}", output);
    println!("{}", output);
    output.parse().unwrap()
}
