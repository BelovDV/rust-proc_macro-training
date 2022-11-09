use proc_macro::TokenStream;
use quote::format_ident;
use syn::{DataStruct, Ident, Type};

pub fn generate(ident: Ident, s: DataStruct) -> TokenStream {
    let DataStruct { fields, .. } = s;
    let named: Vec<_> = fields
        .into_iter()
        .filter_map(|f| Field::try_from(f))
        .collect();
    let res_ty: Vec<_> = named.iter().map(|f| f.ty.clone()).collect();
    let res_ident: Vec<_> = named.iter().map(|f| f.id.clone()).collect();
    let res_ident_str: Vec<_> = named.iter().map(|f| f.id.to_string()).collect();

    let res_ident_types = format_ident!("Reflection{}Fields", ident.to_string());
    quote::quote!(
        #[derive(Debug)]
        pub enum #res_ident_types<'a> {
            ErrorType,
            #(
                #[allow(non_camel_case_types)]
                #res_ident(&'a mut #res_ty)
            ),*
        }

        impl<'a> Reflection<'a> for #ident {
            type Field = #res_ident_types<'a>;

            fn get_field(&'a mut self, name: &str) -> Self::Field {
                match name {
                    #(#res_ident_str => Self::Field:: #res_ident(&mut self.#res_ident),)*
                    _ => Self::Field::ErrorType,
                }
            }

            fn get_field_string(&self, name: &str) -> Result<String, ()> {
                match name {
                    #(#res_ident_str => Ok(format!("{:?}", self.#res_ident)),)*
                    _ => Err(()),
                }
            }

            fn get_field_list() -> Vec<String> {
                vec![#(#res_ident_str.to_string()),*]
            }
        }
    )
    .into()
}

#[derive(Debug, Clone)]
struct Field {
    id: Ident,
    ty: Type,
}

impl Field {
    fn try_from(f: syn::Field) -> Option<Self> {
        match f.ident.clone() {
            Some(id) => Some(Field { id, ty: f.ty }),
            None => None,
        }
    }
}
