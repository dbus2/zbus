#![deny(rust_2018_idioms)]
#![doc(
    html_logo_url = "https://storage.googleapis.com/fdo-gitlab-uploads/project/avatar/3213/zbus-logomark.png"
)]
#![doc = include_str!("../README.md")]

#[cfg(doctest)]
mod doctests {
    doc_comment::doctest!("../README.md");
}

use proc_macro::TokenStream;

mod signature;

/// Macro to create [`zvariant::Signature`] instance validated at compile-time.
///
/// ```rust
/// use core::convert::TryFrom;
///
/// let signature = zvariant_macros::signature!("ss");
/// assert_eq!(signature, zvariant::Signature::try_from("ss").unwrap());
/// ```
#[proc_macro]
pub fn signature(input: TokenStream) -> TokenStream {
    let signature_literal = syn::parse_macro_input!(input as syn::LitStr);
    signature::expand(signature_literal).unwrap_or_else(|err| err.to_compile_error().into())
}
