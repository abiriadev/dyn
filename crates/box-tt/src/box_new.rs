use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, Data, DeriveInput, Field, Fields};

pub fn box_new(input: TokenStream) -> TokenStream {
	let input = match parse2::<DeriveInput>(input) {
		Ok(v) => v,
		Err(e) => return e.to_compile_error(),
	};

	println!("{input:#?}");

	let ident = input.ident;

	let Data::Struct(s) = input.data else {
		panic!()
	};

	let Fields::Named(named) = s.fields else {
		panic!()
	};

	let fields =
		named
			.named
			.iter()
			.filter_map(|Field { ident, ty, .. }| {
				if let Some(i) = ident {
					Some((i, ty))
				} else {
					None
				}
			});

	let args = fields
		.clone()
		.map(|(ident, ty)| quote! { #ident: #ty });

	println!(
		"{:#?}",
		fields.clone().collect::<Vec<_>>()
	);

	let body = fields.map(|(ident, _)| quote! { #ident: #ident});

	quote! {
		impl #ident {
			fn new_box(#(#args),*) -> Self {
				Self {
					#(#body),*
				}
			}
		}
	}
}
