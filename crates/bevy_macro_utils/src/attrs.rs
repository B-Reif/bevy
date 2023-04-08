use crate::symbol::Symbol;
use quote::ToTokens;
use syn::{self, Expr, ExprLit, Lit};

pub fn get_lit_str(attr_name: Symbol, lit: &syn::Lit) -> syn::Result<&syn::LitStr> {
    if let syn::Lit::Str(lit) = lit {
        Ok(lit)
    } else {
        Err(syn::Error::new_spanned(
            lit,
            format!("expected {attr_name} attribute to be a string: `{attr_name} = \"...\"`"),
        ))
    }
}

/// Extract `value` from a name/value pair where the value is a string literal.
///
/// #[foo = bar]
///         ^^^
pub fn get_lit_str_value(meta_name_value: &syn::MetaNameValue) -> syn::Result<&syn::LitStr> {
    match &meta_name_value.value {
        Expr::Lit(ExprLit {
            lit: Lit::Str(s), ..
        }) => Ok(s),
        expr => Err(syn::Error::new_spanned(
            expr.to_token_stream(),
            format!(
                "expected attribute to be a string: `{0} = \"...\"`",
                meta_name_value.path.to_token_stream()
            ),
        )),
    }
}

pub fn get_lit_bool(attr_name: Symbol, lit: &syn::Lit) -> syn::Result<bool> {
    if let syn::Lit::Bool(lit) = lit {
        Ok(lit.value())
    } else {
        Err(syn::Error::new_spanned(
            lit,
            format!("expected {attr_name} attribute to be a bool value, `true` or `false`: `{attr_name} = ...`"),
        ))
    }
}

/// Extract `value` from a name/value pair where the value is a bool literal.
///
/// #[foo = true]
///         ^^^^
pub fn get_lit_bool_value(meta_name_value: &syn::MetaNameValue) -> syn::Result<bool> {
    match &meta_name_value.value {
        Expr::Lit(ExprLit {
            lit: Lit::Bool(b), ..
        }) => Ok(b.value()),
        expr => Err(syn::Error::new_spanned(
            expr.to_token_stream(),
            format!(
                "expected attribute to be a string: `{0} = \"...\"`",
                meta_name_value.path.to_token_stream()
            ),
        )),
    }
}
