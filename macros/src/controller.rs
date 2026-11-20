use quote::{format_ident, quote};
use syn::{ImplItem, ItemImpl};

mod arguments;
mod layers;
mod openapi;
pub(super) mod options;
mod output;
mod route;
mod validation;

pub(super) fn expand_controller(
    options: &options::ControllerOptions, implementation: &mut ItemImpl,
) -> syn::Result<proc_macro2::TokenStream> {
    let prefix = options.route_prefix();
    validation::validate_declaration(&prefix, implementation)?;

    let mut registrations = Vec::new();
    let mut openapi_names = Vec::new();
    let mut openapi_paths = Vec::new();

    for item in &mut implementation.items {
        let ImplItem::Fn(method) = item else { continue };
        let routes = crate::routes::take_routes(method)?;

        if routes.is_empty() {
            continue;
        }

        let layers = layers::take(method)?;
        let signature = &method.sig;
        validation::validate_method(signature)?;

        let (arguments, parameters) = arguments::handler(signature);
        let name = &signature.ident;
        let openapi = openapi::take_path(&mut method.attrs);
        openapi::validate_routes(openapi.as_ref(), routes.len(), signature)?;

        let call = quote!(controller.#name(#(#arguments),*));
        let call = if signature.asyncness.is_some() { quote!(#call.await) } else { call };

        for (verb, path) in routes {
            if !path.value().starts_with('/') {
                return Err(syn::Error::new_spanned(path, "route path must start with /"));
            }

            let full_path = format!("{}{}", prefix.trim_end_matches('/'), path.value());
            let verb = format_ident!("{verb}");
            let guards = options.guards.iter().chain(&layers.guards);
            let interceptors = options.interceptors.iter().chain(&layers.interceptors);
            let pipes = options.pipes.iter().chain(&layers.pipes);
            let filters = options.filters.iter().chain(&layers.filters);
            let middleware = options.middleware.iter().chain(&layers.middleware);
            let metadata_keys = layers.metadata.iter().map(|(key, _)| key);
            let metadata_values = layers.metadata.iter().map(|(_, value)| value);

            if let Some(attribute) = &openapi {
                let path = syn::LitStr::new(&full_path, path.span());
                let attribute = openapi::complete_path(attribute, &verb, &path, options)?;
                openapi_paths.push(quote! {
                    #attribute
                    fn #name() {}

                    const _: fn() = #name;
                });
                openapi_names.push(name.clone());
            }

            registrations.push(route::register(
                &full_path,
                &verb,
                name,
                &parameters,
                &call,
                guards,
                interceptors,
                pipes,
                filters,
                middleware,
                metadata_keys,
                metadata_values,
            ));
        }
    }

    let ty = &implementation.self_ty;
    let (documentation, document) = openapi::document(options, ty, &openapi_names)?;

    Ok(output::controller(
        implementation,
        ty,
        documentation,
        openapi_paths,
        registrations,
        document,
    ))
}
