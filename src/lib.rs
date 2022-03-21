extern crate proc_macro;
use proc_macro::{TokenStream};
use quote::{quote};

#[proc_macro_attribute]
pub fn require_feature(_meta: proc_macro::TokenStream, _input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    TokenStream::from(quote!{
        struct Iv {}
    })
}
