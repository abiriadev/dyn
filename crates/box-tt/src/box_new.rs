use either::Either::{self, Left, Right};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
	parse2, AngleBracketedGenericArguments, Data, DeriveInput, Field, Fields,
	GenericArgument, Path, PathArguments, PathSegment, Type, TypePath,
};

fn unwrap_box(ty: &Type) -> Either<&Type, &Type> {
	let Type::Path(TypePath {
		path: Path { segments, .. },
		..
	}) = ty
	else {
		return Left(ty);
	};

	let Some(PathSegment {
		ident,
		arguments:
			PathArguments::AngleBracketed(AngleBracketedGenericArguments {
				args,
				..
			}),
	}) = segments.first()
	else {
		return Left(ty);
	};

	if ident.to_string() != "Box" {
		return Left(ty);
	};

	let Some(GenericArgument::Type(ty)) = args.first() else {
		return Left(ty);
	};

	Right(ty)
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

	let args = fields.clone().map(|(ident, ty)| {
		let ty = ty.into_inner();

		quote! { #ident: #ty }
	});

	let body = fields.map(|(ident, ty)| {
		ty.map_either(
			|_| quote! {#ident},
			|_| quote! {#ident: Box::new(#ident)},
		)
		.into_inner()
	});

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
