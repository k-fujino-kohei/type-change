mod derive;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(From, attributes(from))]
pub fn derive_typechange(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let token = derive::derive(ast);
    token.into()
}
