mod newtype;
mod utils;

use quote::ToTokens;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Newtype, attributes(newtype))]
pub fn drive_newtype(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match newtype::Newtype::from_derive_input(&parse_macro_input!(input as DeriveInput)) {
        Ok(ok) => ok.into_token_stream().into(),
        Err(err) => err.into_compile_error().into(),
    }
}
