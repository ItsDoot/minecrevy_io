use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;

pub fn get_crate_ident(sub: &Ident) -> TokenStream {
    let found: FoundCrate = crate_name("minecrevy_io").expect("failed to find any crate name");

    match found {
        FoundCrate::Itself => quote! { crate::#sub },
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote! { #ident::#sub }
        }
    }
}
