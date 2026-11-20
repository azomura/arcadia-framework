use quote::{format_ident, quote};
use syn::{FnArg, Signature};

pub(super) fn handler(signature: &Signature) -> (Vec<syn::Ident>, Vec<proc_macro2::TokenStream>) {
    let mut arguments = Vec::new();
    let mut parameters = Vec::new();
    for (index, argument) in signature.inputs.iter().skip(1).enumerate() {
        let FnArg::Typed(argument) = argument else { unreachable!() };
        let name = format_ident!("argument_{index}");
        let ty = &argument.ty;
        parameters.push(quote!(#name: #ty));
        arguments.push(name);
    }
    (arguments, parameters)
}
