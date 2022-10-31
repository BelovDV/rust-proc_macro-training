#![feature(proc_macro_diagnostic)]

use std::vec;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::*;

#[proc_macro_derive(FieldsFromStrings)]
pub fn proc_macro_fields_from_strings(input: TokenStream) -> TokenStream {
    let DeriveInput {
        // attrs,
        // vis,
        ident,
        // generics,
        data,
        ..
    } = parse_macro_input!(input as DeriveInput);

    match data {
        Data::Struct(s) => proc_macro_fields_from_strings_struct(ident, s),
        _ => {
            ident.span().unwrap().error("struct expected").emit();
            "".parse().unwrap()
        }
    }
}

fn proc_macro_fields_from_strings_struct(ident: Ident, s: DataStruct) -> TokenStream {
    let DataStruct {
        // struct_token,
        fields,
        ..
    } = s;
    let mut test: Vec<proc_macro2::TokenStream> = Default::default();

    for field in fields {
        if let Some(fi) = field.ident {
            test.push(
                format!(
                    "\"{0}\" => {{
                        self.{0} = value.parse::<{1}>().map_err(|e| e.to_string())?;
                        Ok(())
                    }}",
                    fi.to_string(),
                    field.ty.to_token_stream().to_string(),
                )
                .parse()
                .unwrap(),
            );
        }
    }

    let _vsp = quote::quote!(fn abc);
    let _vsp = quote::quote!(#_vsp);
    let _vsp = vec![quote::quote!(fn abc), quote::quote!(fn abc)];
    let _vsp = quote::quote!(#(#_vsp)*);

    let res = quote::quote!(
        impl #ident {
            fn set_from_string(&mut self, field: &str, value: &str) -> Result<(), String> {
                match field {
                    #(#test)*
                    other => Err(format!("Wrong field name {}", other)),
                }
            }
        }
    );
    res.into()
}
