use crate::svd::Peripheral;
use crate::util::{self, ToSanitizedUpperCase};

use cast::u64;
use std::collections::HashMap;
use std::fmt::Write;

use anyhow::Result;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

pub fn render(peripherals: &[Peripheral], device_x: &mut String) -> Result<TokenStream> {
    let interrupts = peripherals
        .iter()
        .flat_map(|p| p.interrupt.iter())
        .map(|i| (i.value, i))
        .collect::<HashMap<_, _>>();

    let mut interrupts = interrupts.into_iter().map(|(_, v)| v).collect::<Vec<_>>();
    interrupts.sort_by_key(|i| i.value);

    let mut root = TokenStream::new();
    let mut from_arms = TokenStream::new();
    let mut elements = TokenStream::new();
    let mut names = vec![];
    let mut variants = TokenStream::new();

    // Current position in the vector table
    let mut pos = 0;
    for interrupt in &interrupts {
        while pos < interrupt.value {
            elements.extend(quote!(Vector { _reserved: 0 },));
            pos += 1;
        }
        pos += 1;

        let name_uc = Ident::new(&interrupt.name.to_sanitized_upper_case(), Span::call_site());
        let description = format!(
            "{} - {}",
            interrupt.value,
            interrupt
                .description
                .as_ref()
                .map(|s| util::respace(s))
                .as_ref()
                .map(|s| util::escape_brackets(s))
                .unwrap_or_else(|| interrupt.name.clone())
        );

        let value = util::unsuffixed(u64(interrupt.value));

        variants.extend(quote! {
            #[doc = #description]
            #name_uc = #value,
        });

        from_arms.extend(quote! {
            #value => Ok(Interrupt::#name_uc),
        });

        elements.extend(quote!(Vector { _handler: #name_uc },));
        names.push(name_uc);
    }

    let n = util::unsuffixed(u64(pos));
    for name in &names {
        writeln!(device_x, "PROVIDE({} = DefaultHandler);", name)?;
    }

    root.extend(quote! {
        #[cfg(feature = "rt")]
        extern "C" {
            #(fn #names();)*
        }

        #[doc(hidden)]
        pub union Vector {
            _handler: unsafe extern "C" fn(),
            _reserved: u32,
        }

        #[cfg(feature = "rt")]
        #[doc(hidden)]
        #[link_section = ".vector_table.interrupts"]
        #[no_mangle]
        pub static __INTERRUPTS: [Vector; #n] = [
            #elements
        ];
    });

    let self_token = quote!(self);
    let (enum_repr, nr_expr) = if variants.is_empty() {
        (quote!(), quote!(match *#self_token {}))
    } else {
        (quote!(#[repr(u8)]), quote!(*#self_token as u8))
    };

    let interrupt_enum = quote! {
        ///Enumeration of all the interrupts
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        #enum_repr
        pub enum Interrupt {
            #variants
        }

        unsafe impl bare_metal::Nr for Interrupt {
            #[inline(always)]
            fn nr(&#self_token) -> u8 {
                #nr_expr
            }
        }
    };

    root.extend(interrupt_enum);

    Ok(root)
}
