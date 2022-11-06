use proc_macro::TokenStream;
use syn::*;

pub fn generate(ident: Ident, s: DataStruct) -> TokenStream {
    let DataStruct { fields, .. } = s;
    let mut field_setters: Vec<proc_macro2::TokenStream> = Default::default();
    for field in fields {
        if let Some(fi) = field.ident {
            let ty = field.ty.clone();
            let fi_n = fi.to_string();
            let vsp = quote::quote!(
                #fi_n => {
                    self.#fi = value.parse::<#ty>().map_err(|e| e.to_string())?;
                    Ok(())
                }
            );
            field_setters.push(vsp);
        }
    }

    let res = quote::quote!(
        impl #ident {
            fn set_from_string(&mut self, field: &str, value: &str) -> Result<(), String> {
                match field {
                    #(#field_setters)*
                    other => Err(format!("Wrong field name '{}'", other)),
                }
            }
        }
    );
    res.into()
}
