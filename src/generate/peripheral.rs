use crate::svd::{derive_from::DeriveFrom, Peripheral, RegisterProperties};
use crate::util::{self, ToSanitizedPascalCase, ToSanitizedSnakeCase, ToSanitizedUpperCase};

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use anyhow::Result;

#[derive(Default)]
pub struct PeripheralRendered {
    pub registers: TokenStream,
    pub address: TokenStream,
    pub namespace: String,
}

pub fn render(
    p_original: &Peripheral,
    all_peripherals: &[Peripheral],
    _defaults: &RegisterProperties,
) -> Result<PeripheralRendered> {
    let mut address = TokenStream::new();
    let registers = TokenStream::new();
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
        return Ok(PeripheralRendered::default());
    }

    let span = Span::call_site();
    let name_pc = Ident::new(&p.name.to_sanitized_upper_case_with_suffix("_base"), span);
    let peripheral_address = util::hex(p.base_address as u64);
    let _description = util::respace(p.description.as_ref().unwrap_or(&p.name));

    let _name_sc = Ident::new(&p.name.to_sanitized_snake_case(), span);
    let (_derive_regs, base_regs) = if let (Some(df), None) = (p_derivedfrom, &p_original.registers)
    {
        (true, df)
    } else {
        (false, p)
    };

    let namespace = if let Some(name) = &base_regs.group_name {
        name.clone()
    } else {
        base_regs.name.clone()
    };

    let name_base_regs = Ident::new(
        &base_regs
            .name
            .to_sanitized_pascal_case_with_suffix("Registers"),
        span,
    );

    // Insert the peripheral structure
    address.extend(quote! {
        pub const #name_pc: *const #name_base_regs = #peripheral_address as *const _;
    });

    Ok(PeripheralRendered {
        registers,
        address,
        namespace,
    })
}
