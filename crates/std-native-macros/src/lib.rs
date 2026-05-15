mod macro_impl;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn native_plugin(attrs: TokenStream, input: TokenStream) -> TokenStream {
    macro_impl::native_plugin(attrs.into(), input.into())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}