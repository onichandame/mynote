use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::crud::helper::{
    get_meta_field, get_meta_list, get_metas, get_object_name, get_struct_fields,
};

pub fn filter_expand(input: &DeriveInput) -> TokenStream {
    let name = get_object_name(input.attrs.clone(), "filter").map_or(
        format!("{}Filter", input.ident.to_string())
            .parse()
            .unwrap(),
        |v| v,
    );
    let fields = get_struct_fields(&input.data).unwrap();
    let filter_body: Vec<TokenStream> = fields
        .clone()
        .into_iter()
        .filter_map(|v| {
            let name = v.ident.expect("fields must be named");
            if let Ok(meta) = get_meta_list(v.attrs, "crud") {
                if let Ok(filter) = get_meta_field(&meta, "filter") {
                    Some(quote! {
                        #name: Option<#filter>
                    })
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    let model = get_meta_field(&get_metas(&input.attrs).unwrap(), "model").unwrap();
    let build: Vec<TokenStream> = fields
        .clone()
        .into_iter()
        .filter_map(|v| {
            let name = v.ident.expect("fields must be named");
            if let Ok(meta) = get_meta_list(v.attrs, "crud") {
                if let Ok(col) = get_meta_field(&meta, "column") {
                    let col: TokenStream =
                        format!("{}::Column::{}", model.to_string(), col.to_string())
                            .parse()
                            .unwrap();
                    Some(quote! {
                        if let Some(v)=&self.#name{
                            filter=filter.add(v.build(#col));
                        }
                    })
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    quote! {
        #[derive(async_graphql::InputObject, Default)]
        pub struct #name {
            #(#filter_body),*
        }

        impl #name {
            pub fn build(&self)->sea_orm::Condition{
                let mut filter = sea_orm::Condition::all();
                #(#build)*
                filter
            }
        }
    }
}
