use proc_macro::TokenStream;

mod box_new;

#[proc_macro_derive(BoxNew)]
pub fn box_new(input: TokenStream) -> TokenStream { box_new::box_new(input.into()).into() }
