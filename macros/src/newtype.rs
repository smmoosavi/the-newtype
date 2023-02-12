use crate::utils::crate_name::get_crate_name;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::DeriveInput;

pub struct Newtype {
    ident: syn::Ident,
    generics: syn::Generics,
    ty: syn::Type,
}

fn get_type(data: &syn::Data, ident: &syn::Ident) -> Result<syn::Type, syn::Error> {
    match data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => {
            if fields.len() > 1 {
                return Err(syn::Error::new_spanned(
                    fields.iter().nth(1).unwrap(),
                    "Newtype must have exactly one field",
                ));
            }

            let field = fields.iter().next().ok_or_else(|| {
                syn::Error::new_spanned(ident, "Newtype must have exactly one field")
            })?;
            Ok(field.ty.clone())
        }
        _ => Err(syn::Error::new_spanned(
            ident,
            "Newtype can only be derived for structs",
        )),
    }
}

impl Newtype {
    pub fn from_derive_input(input: &DeriveInput) -> Result<Self, syn::Error> {
        let ident = input.ident.clone();
        let generics = input.generics.clone();
        let data = input.data.clone();
        let ty = get_type(&data, &ident)?;
        Ok(Self {
            ident,
            generics,
            ty,
        })
    }
}

impl ToTokens for Newtype {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let crate_name = get_crate_name();
        let ident = &self.ident;
        let generics = &self.generics;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
        let ty = &self.ty;

        let impl_newtype = quote! {
            impl #impl_generics #crate_name::Newtype for #ident #ty_generics #where_clause {
                type Inner = #ty;
            }
        };

        tokens.extend(impl_newtype);
    }
}
