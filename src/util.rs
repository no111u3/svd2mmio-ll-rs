use inflections::Inflect;
use proc_macro2::{Literal, TokenStream};
use quote::ToTokens;
use std::borrow::Cow;

/// List of chars that some vendors use in their peripheral/field names but
/// that are not valid in Rust ident
const BLACKLIST_CHARS: &[char] = &['(', ')', '[', ']', '/', ' ', '-'];

/// Turns `n` into an unsuffixed token
pub fn unsuffixed(n: u64) -> TokenStream {
    Literal::u64_unsuffixed(n).into_token_stream()
}

pub trait ToSanitizedUpperCase {
    fn to_sanitized_upper_case(&self) -> Cow<str>;
}

impl ToSanitizedUpperCase for str {
    fn to_sanitized_upper_case(&self) -> Cow<str> {
        let s = self.replace(BLACKLIST_CHARS, "");

        match s.chars().next().unwrap_or('\0') {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                Cow::from(format!("_{}", s.to_upper_case()))
            }
            _ => Cow::from(s.to_upper_case()),
        }
    }
}

pub fn respace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn escape_brackets(s: &str) -> String {
    s.split('[')
        .fold("".to_string(), |acc, x| {
            if acc == "" {
                x.to_string()
            } else if acc.ends_with('\\') {
                acc + "[" + x
            } else {
                acc + "\\[" + x
            }
        })
        .split(']')
        .fold("".to_string(), |acc, x| {
            if acc == "" {
                x.to_string()
            } else if acc.ends_with('\\') {
                acc + "]" + x
            } else {
                acc + "\\]" + x
            }
        })
}
