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

#[cfg(test)]
mod tests {
    use syn::parse_str;

    use super::handler;

    #[test]
    fn maps_handler_parameters_to_generated_argument_names() {
        let signature: syn::Signature =
            parse_str("fn route(&self, id: u64, body: Payload)").unwrap();
        let (arguments, parameters) = handler(&signature);
        let rendered = parameters.iter().map(ToString::to_string).collect::<Vec<_>>();

        assert_eq!(
            arguments.iter().map(ToString::to_string).collect::<Vec<_>>(),
            ["argument_0", "argument_1"]
        );
        assert_eq!(rendered, ["argument_0 : u64", "argument_1 : Payload"]);
    }
}
