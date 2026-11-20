use quote::quote;
use syn::{ItemImpl, Type};

pub(super) fn controller(
    implementation: &ItemImpl, ty: &Type, documentation: proc_macro2::TokenStream,
    paths: Vec<proc_macro2::TokenStream>, routes: Vec<proc_macro2::TokenStream>,
    document: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    quote! {
        #implementation
        #documentation
        #(#paths)*
        impl ::arcadia_framework::Controller for #ty {
            fn routes(self) -> ::axum::Router {
                Self::build_routes(self, ::arcadia_framework::Container::default(), false)
            }

            fn routes_with_container(
                self, container: ::arcadia_framework::Container,
            ) -> ::axum::Router {
                Self::build_routes(self, container, true)
            }

            fn openapi() -> ::utoipa::openapi::OpenApi {
                #document
            }
        }
        impl #ty {
            fn build_routes(
                self, container: ::arcadia_framework::Container, request_scope: bool,
            ) -> ::axum::Router
            where Self: Clone + Send + Sync + 'static {
                let _ = (&self, &container, request_scope);
                let mut router = ::axum::Router::new();
                #(router = router.merge(#routes);)*
                router
            }
        }
    }
}
