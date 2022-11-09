use proc_macro::TokenStream;
use syn::{DataStruct, Ident};

pub fn generate(ident: Ident, s: DataStruct) -> TokenStream {
    let DataStruct { fields, .. } = s;
    let named: Vec<_> = fields
        .into_iter()
        .filter_map(|f| Field::try_from(f))
        .collect();
    let res_ident = named.iter().map(|f| f.id.clone()).collect::<Vec<_>>();
    let res_ident_str = named.iter().map(|f| f.id.to_string()).collect::<Vec<_>>();

    let res = quote::quote!(
        impl UpdateWithStr for #ident {
            type Err = String;

            fn update_with_str(&mut self, s: &str) -> Result<(), Self::Err> {
                match s.find(|c: char| c.is_whitespace()) {
                    Some(pos) => {
                        match &s[..pos] {
                            #(#res_ident_str => {
                                match s[pos..].trim() {
                                    "" => Err("list of args is ended".to_string()),
                                    s => self.#res_ident.update_with_str(s).map_err(|e| e.to_string())
                                }
                            }),*
                            _ => Err(format!("wrong field name `{}`", &s[..pos]))
                        }
                    }
                    None => Err("cannot find value (split by whitespace".to_string())
                }
            }
        }
    );
    println!("\n\n\n");
    dbg!(res.to_string());
    println!("\n\n\n");
    res.into()
}

#[derive(Debug, Clone)]
struct Field {
    id: Ident,
}

impl Field {
    fn try_from(f: syn::Field) -> Option<Self> {
        match f.ident.clone() {
            Some(id) => Some(Field { id }),
            None => None,
        }
    }
}
