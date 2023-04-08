use bevy_macro_utils::Symbol;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, parse_quote, DeriveInput, Error, Ident, LitStr, Path, Result};

pub fn derive_resource(input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    let bevy_ecs_path: Path = crate::bevy_ecs_path();

    ast.generics
        .make_where_clause()
        .predicates
        .push(parse_quote! { Self: Send + Sync + 'static });

    let struct_name = &ast.ident;
    let (impl_generics, type_generics, where_clause) = &ast.generics.split_for_impl();

    TokenStream::from(quote! {
        impl #impl_generics #bevy_ecs_path::system::Resource for #struct_name #type_generics #where_clause {
        }
    })
}

pub fn derive_component(input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    let bevy_ecs_path: Path = crate::bevy_ecs_path();

    let attrs = match parse_component_attr(&ast) {
        Ok(attrs) => attrs,
        Err(e) => return e.into_compile_error().into(),
    };

    let storage = storage_path(&bevy_ecs_path, attrs.storage);

    ast.generics
        .make_where_clause()
        .predicates
        .push(parse_quote! { Self: Send + Sync + 'static });

    let struct_name = &ast.ident;
    let (impl_generics, type_generics, where_clause) = &ast.generics.split_for_impl();

    TokenStream::from(quote! {
        impl #impl_generics #bevy_ecs_path::component::Component for #struct_name #type_generics #where_clause {
            type Storage = #storage;
        }
    })
}

pub const COMPONENT: Symbol = Symbol("component");
pub const STORAGE: Symbol = Symbol("storage");

struct Attrs {
    storage: StorageTy,
}

#[derive(Clone, Copy)]
enum StorageTy {
    Table,
    SparseSet,
}

// values for `storage` attribute
const TABLE: &str = "Table";
const SPARSE_SET: &str = "SparseSet";

fn parse_component_attr(ast: &DeriveInput) -> Result<Attrs> {
    let mut attrs = Attrs {
        storage: StorageTy::Table,
    };

    // Parses #[component(...)] attributes.
    for attr in ast.attrs.iter().filter(|a| a.path().is_ident(&COMPONENT)) {
        attr.parse_nested_meta(|meta| {
            // Parses #[component(storage)]
            if meta.path.is_ident(&STORAGE) {
                let content = meta.value()?;
                let lit: LitStr = content.parse()?;
                attrs.storage = match lit.parse::<Path>() {
                    Ok(path) if path.is_ident(&TABLE) => StorageTy::Table,
                    Ok(path) if path.is_ident(&SPARSE_SET) => StorageTy::SparseSet,
                    Ok(path) => {
                        let found = path.to_token_stream();
                        return Err(Error::new_spanned(
                            path,
                            format!(
                                "Invalid storage type '{found}', expected '{TABLE}' or '{SPARSE_SET}'.",
                            ),
                        ));
                    }
                    _ => todo!(),
                };
                Ok(())
            } else {
                let found = meta.path.to_token_stream();
                Err(Error::new_spanned(
                    meta.path,
                    format!(
                        "unknown component attribute `{}`",
                        found
                    ),
                ))
            }
        })?;
    }

    Ok(attrs)
}

fn storage_path(bevy_ecs_path: &Path, ty: StorageTy) -> TokenStream2 {
    let typename = match ty {
        StorageTy::Table => Ident::new("TableStorage", Span::call_site()),
        StorageTy::SparseSet => Ident::new("SparseStorage", Span::call_site()),
    };

    quote! { #bevy_ecs_path::component::#typename }
}
