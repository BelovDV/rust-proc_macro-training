#![feature(proc_macro_diagnostic)]

use proc_macro::TokenStream;
use syn::*;

mod enum_from_strings;
mod fields_from_strings;
mod reflection;
mod update_with_str;

#[proc_macro_derive(FieldsFromStrings)]
pub fn proc_macro_fields_from_strings(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);
    match data {
        Data::Struct(s) => fields_from_strings::generate(ident, s),
        _ => {
            ident.span().unwrap().error("struct expected").emit();
            "".parse().unwrap()
        }
    }
}

#[proc_macro_derive(EnumFromStrings)]
pub fn proc_macro_enum_from_strings(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);
    match data {
        Data::Enum(e) => enum_from_strings::generate(ident, e),
        _ => {
            ident.span().unwrap().error("struct expected").emit();
            "".parse().unwrap()
        }
    }
}

#[proc_macro_derive(Reflection)]
pub fn proc_macro_reflection(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);
    match data {
        Data::Struct(s) => reflection::generate(ident, s),
        _ => {
            ident.span().unwrap().error("struct expected").emit();
            "".parse().unwrap()
        }
    }
}

#[proc_macro_derive(UpdateWithStr)]
pub fn proc_macro_update_with_str(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);
    match data {
        Data::Struct(s) => update_with_str::generate(ident, s),
        _ => {
            ident.span().unwrap().error("struct expected").emit();
            "".parse().unwrap()
        }
    }
}
