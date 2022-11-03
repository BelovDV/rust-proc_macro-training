use proc_macro::TokenStream;
use syn::*;

pub fn generate(ident: Ident, e: DataEnum) -> TokenStream {
    let DataEnum { variants, .. } = e;
    let mut correct: Vec<proc_macro2::TokenStream> = Default::default();
    for var in variants {
        if var.fields == Fields::Unit {
            let fi = var.ident;
            let fi_n = fi.to_string();
            correct.push(quote::quote!(#fi_n => Ok(#ident::#fi),));
        } else {
            ident.span().unwrap().error("unit only").emit()
        }
    }

    let res = quote::quote!(
        impl ::std::str::FromStr for #ident {
            type Err = &'static str;
            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                match s {
                    #(#correct)*
                    _ => Err("wrong string")
                }
            }
        }
    );
    res.into()
}
