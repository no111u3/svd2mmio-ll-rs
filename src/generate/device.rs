use crate::svd::Device;
use anyhow::Result;

use proc_macro2::TokenStream;

pub fn render(_d: &Device) -> Result<TokenStream> {
    let out = TokenStream::new();

    Ok(out)
}
