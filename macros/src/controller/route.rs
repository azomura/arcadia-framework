use quote::quote;
use syn::LitStr;

#[allow(clippy::too_many_arguments)]
pub(super) fn register<'a>(
    path: &str, verb: &syn::Ident, name: &syn::Ident, parameters: &[proc_macro2::TokenStream],
    call: &proc_macro2::TokenStream, guards: impl Iterator<Item = &'a syn::Type>,
    interceptors: impl Iterator<Item = &'a syn::Type>, pipes: impl Iterator<Item = &'a syn::Type>,
    filters: impl Iterator<Item = &'a syn::Type>, middleware: impl Iterator<Item = &'a syn::Type>,
    metadata_keys: impl Iterator<Item = &'a LitStr>,
    metadata_values: impl Iterator<Item = &'a LitStr>,
) -> proc_macro2::TokenStream {
    quote! {{
        let route = ::axum::Router::new().route(#path, ::axum::routing::#verb(
            move |::axum::extract::State(controller): ::axum::extract::State<Self>,
             ::arcadia_framework::RequestContainer(request_container),
             #(#parameters),*| async move {
                let controller = if request_scope {
                    (*request_container.resolve::<Self>()).clone()
                } else {
                    controller
                };
                #call
            }
        )).with_state(self.clone());
        let route = route.layer(::axum::middleware::from_fn(
            ::arcadia_framework::application::request::context,
        )).layer(::axum::Extension(container.clone()));
        let metadata = ::arcadia_framework::http::RouteMetadata::new(
            ::std::any::type_name::<Self>(), stringify!(#name),
        )#(.with(#metadata_keys, #metadata_values))*;
        let pipeline = ::arcadia_framework::http::HttpPipeline::new()
            #(.guard(<#guards as ::std::default::Default>::default()))*
            #(.interceptor(<#interceptors as ::std::default::Default>::default()))*
            #(.pipe(<#pipes as ::std::default::Default>::default()))*
            #(.filter(<#filters as ::std::default::Default>::default()))*
            #(.middleware(<#middleware as ::std::default::Default>::default()))*;
        pipeline.apply(route).layer(::axum::Extension(metadata))
    }}
}
