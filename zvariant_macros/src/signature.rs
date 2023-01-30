use core::convert::TryFrom;
use proc_macro::TokenStream;
use syn::{Error, LitStr};
use zvariant::Signature;

pub fn expand(input: LitStr) -> syn::Result<TokenStream> {
    let raw_sig = input.value();

    // Create a Signature instance to make sure the input passes all validations.
    if let Err(err) = Signature::try_from(raw_sig.clone()) {
        return Err(Error::new(input.span(), err));
    };

    // Okay, now we know it's fine to use this one unchecked.
    let tokens = quote::quote! {
        zvariant::Signature::from_static_str_unchecked(#raw_sig)
    };

    Ok(tokens.into())
}
