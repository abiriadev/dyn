use proc_macro2::TokenStream;
use quote::quote;
use syn::{
	parse2, Data, DeriveInput, Field, Fields, GenericArgument, PathArguments,
	Type,
};

enum BoxOrNot<T> {
	Box(T),
	Not(T),
}

fn unwrap_box(ty: &Type) -> BoxOrNot<&Type> {
	let Type::Path(path) = ty else {
		return BoxOrNot::Not(ty);
	};

	let Some(seg) = path.path.segments.first() else {
		return BoxOrNot::Not(ty);
	};

	if seg.ident.to_string() != "Box" {
		return BoxOrNot::Not(ty);
	};

	let PathArguments::AngleBracketed(ang) = seg.arguments else {
		return BoxOrNot::Not(ty);
	};

	let Some(GenericArgument::Type(ty)) = ang.args.first() else {
		return BoxOrNot::Not(ty);
	};

	BoxOrNot::Box(ty)
}

pub fn box_new(input: TokenStream) -> TokenStream {
	let input = match parse2::<DeriveInput>(input) {
		Ok(v) => v,
		Err(e) => return e.to_compile_error(),
	};

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
			.filter_map(|Field { ident, ty, .. }| match ident {
				Some(i) => Some((i, unwrap_box(ty))),
				None => None,
			});

	let args = fields.clone().map(|(ident, mut ty)| {
		quote! { #ident: #ty }
	});

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
