use syn::{FnArg, ItemImpl, Signature};

pub(super) fn validate_declaration(prefix: &str, implementation: &ItemImpl) -> syn::Result<()> {
    if implementation.trait_.is_some() || !implementation.generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            implementation,
            "#[controller] requires a non-generic inherent impl",
        ));
    }

    if !prefix.is_empty() && !prefix.starts_with('/') {
        return Err(syn::Error::new_spanned(implementation, "controller prefix must start with /"));
    }

    Ok(())
}

pub(super) fn validate_method(signature: &Signature) -> syn::Result<()> {
    if !signature.generics.params.is_empty() {
        return Err(syn::Error::new_spanned(signature, "route methods cannot be generic"));
    }

    match signature.inputs.first() {
        Some(FnArg::Receiver(receiver))
            if matches!(&receiver.kind, syn::ReceiverKind::Reference(_, _, None)) =>
        {
            Ok(())
        }
        _ => Err(syn::Error::new_spanned(signature, "route methods require &self")),
    }
}
