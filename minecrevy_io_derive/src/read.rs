use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned};
use syn::{Data, DeriveInput, Field, Member, spanned::Spanned};

pub fn gen_impl(input: DeriveInput) -> TokenStream {
    let ident = &input.ident;
    let fields = gen_fields(&input.data);
    let mcread = crate::util::get_crate_ident(&Ident::new("McRead", input.span()));

    quote! {
        impl #mcread for #ident {
            fn read<R: ::std::io::Read>(mut reader: R) -> ::std::io::Result<Self> {
                Ok(Self {
                    #(#fields)*
                })
            }
        }
    }
}

fn gen_fields(data: &Data) -> impl Iterator<Item=TokenStream> + '_ {
    match data {
        Data::Struct(data) => data.fields.iter()
            .enumerate()
            .map(|(idx, field)| {
                let member = field.ident.as_ref()
                    .map(|id| Member::Named(id.clone()))
                    .unwrap_or_else(|| Member::Unnamed(idx.into()));
                gen_field(field, member)
            }),
        _ => unimplemented!(),
    }
}

fn gen_field(field: &Field, ident: Member) -> TokenStream {
    let ty = &field.ty;
    let mcread = crate::util::get_crate_ident(&Ident::new("McRead", field.span()));

    quote_spanned! { field.span() =>
        #ident: <#ty as #mcread>::read(&mut reader)?,
    }
}
