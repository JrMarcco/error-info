use crate::to_error_info::process_to_error_info;
use proc_macro::TokenStream;

mod to_error_info;

#[proc_macro_derive(ToErrorInfo, attributes(error_info))]
pub fn derive_to_error(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    process_to_error_info(input).into()
}
