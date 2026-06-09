use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn async_trait(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
