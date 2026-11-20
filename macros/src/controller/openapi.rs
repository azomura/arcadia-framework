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

#[cfg(test)]
mod tests {
    use quote::{ToTokens, format_ident};
    use syn::{LitStr, parse_quote, parse_str};

    use super::{complete_path, document, take_path, validate_routes};
    use crate::controller::options::ControllerOptions;

    #[test]
    fn extracts_and_completes_openapi_path_attributes() {
        let mut attributes: Vec<syn::Attribute> =
            vec![parse_quote!(#[utoipa::path(responses((status = 200)))])];
        let path = take_path(&mut attributes).unwrap();
        let options = ControllerOptions::default();
        let completed = complete_path(
            &path,
            &format_ident!("get"),
            &LitStr::new("/users", proc_macro2::Span::call_site()),
            &options,
        )
        .unwrap();

        assert!(attributes.is_empty());
        assert!(completed.to_token_stream().to_string().contains("users"));
    }

    #[test]
    fn validates_route_count_and_builds_empty_document() {
        let signature: syn::Signature = parse_str("fn route(&self)").unwrap();
        let attribute: syn::Attribute = parse_quote!(#[utoipa::path(get)]);
        assert!(validate_routes(Some(&attribute), 2, &signature).is_err());
        assert!(validate_routes(Some(&attribute), 1, &signature).is_ok());

        let options = ControllerOptions::default();
        let (declaration, value) = document(&options, &parse_quote!(Controller), &[]).unwrap();
        assert!(declaration.is_empty());
        assert!(value.to_string().contains("OpenApi"));
    }
}
