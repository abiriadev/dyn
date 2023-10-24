use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, DeriveInput};

pub fn box_new(input: TokenStream) -> TokenStream {
	let input = match parse2::<DeriveInput>(input) {
		Ok(v) => v,
		Err(e) => return e.to_compile_error(),
	};

	println!("{input:#?}");

	quote! {
		// impl
	}
}
