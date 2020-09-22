use proc_macro2::{Literal, TokenStream};
use quote::ToTokens;

/// Turns `n` into an unsuffixed token
pub fn unsuffixed(n: u64) -> TokenStream {
    Literal::u64_unsuffixed(n).into_token_stream()
}
