extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro::TokenTree::{Ident, Group};


#[proc_macro_derive(Deserialization)]
pub fn derive_answer_fn(item: TokenStream) -> TokenStream {
    let mut index  = 0;
    let mut output:String = "".to_string();
    for x in item.into_iter() {
        match &x {
            Ident(ident) => {
                index += 1;
                if index == 1 {
                    output = format!("impl Deserialization for {} {{", ident);
                }
            },
            Group(group) => {
                for line in group.stream().into_iter() {
                    println!("Line {:?}", line);

                }
            }
            _ => {}

        };
        println!("{:?}", x);
    }
    println!("{}", output);
    output.parse().unwrap()
}

