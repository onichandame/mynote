use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::crud::helper::{
    get_meta_field, get_meta_list, get_metas, get_object_name, get_struct_fields,
};

pub fn sort_expand(input: &DeriveInput) -> TokenStream {
    let field_name = get_object_name(input.attrs.clone(), "field").map_or(
        format!("{}Field", input.ident.to_string()).parse().unwrap(),
        |v| v,
    );
    let sort_name = get_object_name(input.attrs.clone(), "sort").map_or(
        format!("{}Sort", input.ident.to_string()).parse().unwrap(),
        |v| v,
    );
    let model = get_meta_field(&get_metas(&input.attrs).unwrap(), "model").unwrap();
    let fields = get_struct_fields(&input.data).unwrap();
    let field: Vec<TokenStream> = fields
        .clone()
        .into_iter()
        .filter_map(|v| {
            if let Ok(meta) = get_meta_list(v.attrs, "crud") {
                if let Ok(col) = get_meta_field(&meta, "column") {
                    Some(quote! {#col})
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    let field_into: Vec<TokenStream> = fields
        .clone()
        .into_iter()
        .filter_map(|v| {
            if let Ok(meta) = get_meta_list(v.attrs, "crud") {
                if let Ok(col) = get_meta_field(&meta, "column") {
                    Some(quote! {Self::#col=>#model ::Column:: #col})
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    quote! {
        #[derive(async_graphql::Enum, Clone, Copy, Eq, PartialEq)]
        pub enum #field_name {
            #(#field),*
        }

        #[derive(async_graphql::InputObject)]
        pub struct #sort_name {
            field: #field_name,
            direction: crud::SortDirection,
        }

        impl Into<#model ::Column> for #field_name{
            fn into(self)->#model ::Column{
                match self{
                    #(#field_into),*
                }
            }
        }

        impl #sort_name{
            pub fn apply_sort<TQuery: sea_orm::QueryOrder>(&self, query: TQuery)->TQuery{
                self.direction.apply_sort(query,Into::<#model ::Column>::into(self.field.clone()))
            }
        }
    }
}
