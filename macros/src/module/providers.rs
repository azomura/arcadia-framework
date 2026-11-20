use quote::quote;

use super::options::{Binding, ModuleOptions};

pub(super) fn custom(options: &ModuleOptions) -> Vec<proc_macro2::TokenStream> {
    let mut registrations = options
        .values
        .iter()
        .map(|binding| {
            let token = &binding.token;
            let value = &binding.value;
            quote!(context.value(#token, #value);)
        })
        .collect::<Vec<_>>();
    registrations
        .extend(options.factories.iter().map(|binding| factory(binding, quote!(Singleton))));
    registrations
        .extend(options.request_factories.iter().map(|binding| factory(binding, quote!(Request))));
    registrations.extend(
        options.transient_factories.iter().map(|binding| factory(binding, quote!(Transient))),
    );
    registrations
        .extend(options.token_exports.iter().map(|token| quote!(context.export_token(#token);)));
    registrations
}

fn factory(binding: &Binding, scope: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let token = &binding.token;
    let value = &binding.value;
    quote!(context.factory(#token, ::arcadia_framework::Scope::#scope, #value);)
}
