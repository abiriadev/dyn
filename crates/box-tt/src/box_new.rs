use convert_case::{Case, Casing};
use either::Either::{self, Left, Right};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
	parse2, punctuated::Punctuated, token::Comma,
	AngleBracketedGenericArguments, Data, DataEnum, DataStruct, DeriveInput,
	Field, Fields, FieldsUnnamed, GenericArgument, Path, PathArguments,
	PathSegment, Type, TypePath, Variant,
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

	match input.data {
		Data::Struct(DataStruct { fields, .. }) => box_struct(ident, fields),
		Data::Enum(DataEnum { variants, .. }) => box_enum(ident, variants),
		Data::Union(_) => unimplemented!(),
	}
}

fn box_struct(ident: Ident, fields: Fields) -> TokenStream {
	let Fields::Named(named) = fields else {
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
			pub fn new_box(#(#args),*) -> Self {
				Self {
					#(#body),*
				}
			}
		}
	}
}

fn box_enum(ident: Ident, variants: Punctuated<Variant, Comma>) -> TokenStream {
	let methods =
		variants
			.iter()
			.filter_map(|Variant { ident, fields, .. }| {
				if let Fields::Unnamed(FieldsUnnamed { unnamed, .. }) = fields {
					let method = format_ident!(
						"{}_box",
						ident.to_string().to_case(Case::Snake)
					);

					let fields = unnamed.iter().enumerate().map(
						|(i, Field { ty, .. })| {
							(
								Ident::new(&format!("i{i}"), Span::call_site()),
								unwrap_box(ty),
							)
						},
					);

					let args = fields.clone().map(|(ident, ty)| {
						let ty = ty.into_inner();

						quote! { #ident: #ty }
					});

					let body = fields.map(|(ident, ty)| {
						ty.map_either(
							|_| quote! {#ident},
							|_| quote! {Box::new(#ident)},
						)
						.into_inner()
					});

					Some(quote! {
						pub fn #method(#(#args),*) -> Self {
							Self::#ident(#(#body),*)
						}
					})
				} else {
					None
				}
			});

	quote! {
		impl #ident {
			#(#methods)*
		}
	}
}
