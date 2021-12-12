#![allow(non_snake_case)]
extern crate proc_macro;

use syn::{DeriveInput, parse_macro_input};

mod read;
mod write;
pub(crate) mod util;

/// Example of user-defined [derive mode macro][1]
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#derive-mode-macros
#[proc_macro_derive(McRead)]
pub fn McRead(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    read::gen_impl(input).into()
}
