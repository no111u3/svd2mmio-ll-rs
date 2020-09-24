use crate::svd::{derive_from::DeriveFrom, Peripheral, RegisterProperties};
use crate::util::{self, ToSanitizedPascalCase, ToSanitizedSnakeCase, ToSanitizedUpperCase};

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use anyhow::Result;

pub fn render(
    p_original: &Peripheral,
    all_peripherals: &[Peripheral],
    _defaults: &RegisterProperties,
) -> Result<TokenStream> {
    let mut out = TokenStream::new();
    let p_derivedfrom = p_original
        .derived_from
        .as_ref()
        .and_then(|s| all_peripherals.iter().find(|x| x.name == *s));

    let p_merged = p_derivedfrom.map(|ancestor| p_original.derive_from(ancestor));
    let p = p_merged.as_ref().unwrap_or(p_original);

    if let (Some(df), None) = (p_original.derived_from.as_ref(), &p_derivedfrom) {
        eprintln!(
            "Couldn't find derivedFrom original: {} for {}, skipping",
            df, p_original.name
        );
        return Ok(out);
    }

    let span = Span::call_site();
    let _name_pc = Ident::new(&p.name.to_sanitized_upper_case(), span);
    let address = util::hex(p.base_address as u64);
    let description = util::respace(p.description.as_ref().unwrap_or(&p.name));

    let name_sc = Ident::new(&p.name.to_sanitized_snake_case(), span);
    let (derive_regs, _) = if let (Some(df), None) = (p_derivedfrom, &p_original.registers) {
        (true, Ident::new(&df.name.to_sanitized_snake_case(), span))
    } else {
        (false, name_sc.clone())
    };

    let name_pc_regs = Ident::new(
        &p.name.to_sanitized_pascal_case_with_suffix("Registers"),
        span,
    );

    // Insert the peripheral structure
    out.extend(quote! {
        #[doc = #description]
        #[repl(C)]
        pub struct #name_pc_regs {}

        unsafe impl Send for #name_pc_regs {}

        impl #name_pc_regs {
            ///Pointer to the register block
            pub const PTR: *const #name_pc_regs = #address as *const _;

            ///Return the pointer to the register block
            #[inline(always)]
            pub const fn ptr() -> *const #name_pc_regs {
                Self::PTR
            }
        }

        impl Deref for #name_pc_regs {
            type Target = #name_pc_regs;

            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                unsafe { &*Self::PTR }
            }
        }
    });

    // Derived peripherals may not require re-implementation, and will instead
    // use a single definition of the non-derived version.
    if derive_regs {
        return Ok(out);
    }

    Ok(out)
}
