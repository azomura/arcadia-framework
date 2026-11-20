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

#[cfg(test)]
mod tests {
    use syn::parse_str;

    use super::{validate_declaration, validate_method};

    #[test]
    fn accepts_valid_controller_and_method_declarations() {
        let implementation: syn::ItemImpl =
            parse_str("impl Controller { fn route(&self) {} }").unwrap();
        let method: syn::Signature = parse_str("fn route(&self, value: String)").unwrap();

        assert!(validate_declaration("/api", &implementation).is_ok());
        assert!(validate_method(&method).is_ok());
    }

    #[test]
    fn rejects_invalid_controller_prefix_and_method_shapes() {
        let implementation: syn::ItemImpl =
            parse_str("impl Controller { fn route(&self) {} }").unwrap();
        let generic: syn::ItemImpl =
            parse_str("impl<T> Controller { fn route(&self) {} }").unwrap();
        let method: syn::Signature = parse_str("fn route(self)").unwrap();
        let generic_method: syn::Signature = parse_str("fn route<T>(&self)").unwrap();

        assert!(validate_declaration("api", &implementation).is_err());
        assert!(validate_declaration("/api", &generic).is_err());
        assert!(validate_method(&method).is_err());
        assert!(validate_method(&generic_method).is_err());
    }
}
