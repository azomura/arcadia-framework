use quote::quote;
use syn::{Attribute, Ident, LitStr, Meta, Signature, Type};

use super::options::ControllerOptions;

pub(super) fn take_path(attributes: &mut Vec<Attribute>) -> Option<Attribute> {
    let index = attributes.iter().position(|attribute| {
        let segments = &attribute.path().segments;
        segments.len() == 2 && segments[0].ident == "utoipa" && segments[1].ident == "path"
    })?;

    Some(attributes.remove(index))
}

pub(super) fn validate_routes(
    attribute: Option<&Attribute>, routes: usize, signature: &Signature,
) -> syn::Result<()> {
    if attribute.is_some() && routes > 1 {
        return Err(syn::Error::new_spanned(
            signature,
            "OpenAPI route methods must have exactly one HTTP attribute",
        ));
    }

    Ok(())
}

pub(super) fn complete_path(
    attribute: &Attribute, verb: &Ident, path: &LitStr, options: &ControllerOptions,
) -> syn::Result<Attribute> {
    let Meta::List(metadata) = &attribute.meta else {
        return Err(syn::Error::new_spanned(attribute, "expected OpenAPI path arguments"));
    };

    let arguments = &metadata.tokens;
    let tag = options.tag.as_ref().map(|tag| quote!(tag = #tag,));
    let controller_description = options.description.as_ref();
    let description = controller_description.map(|description| quote!(description = #description,));

    Ok(syn::parse_quote! {
        #[utoipa::path(#verb, path = #path, #tag #description #arguments)]
    })
}

pub(super) fn document(
    options: &ControllerOptions, ty: &Type, paths: &[Ident],
) -> syn::Result<(proc_macro2::TokenStream, proc_macro2::TokenStream)> {
    if paths.is_empty() {
        return Ok((quote!(), quote!(::utoipa::openapi::OpenApi::default())));
    }

    let Type::Path(controller) = ty else {
        return Err(syn::Error::new_spanned(ty, "controller type must be a path"));
    };
    let name = &controller.path.segments.last().unwrap().ident;
    let document = quote::format_ident!("__{name}OpenApi");
    let tags = options.tag.as_ref().map(|tag| {
        let description = options.description.as_ref().unwrap_or(tag);
        quote!(tags((name = #tag, description = #description)),)
    });
    let declaration = quote! {
        #[derive(::utoipa::OpenApi)]
        #[openapi(paths(#(#paths),*), #tags)]
        struct #document;
    };
    let value = quote!(<#document as ::utoipa::OpenApi>::openapi());

    Ok((declaration, value))
}
