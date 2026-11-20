use proc_macro::TokenStream;

use quote::quote;
use syn::{
    Expr, FnArg, GenericArgument, ImplItem, ItemImpl, PathArguments, Type, parse_macro_input,
};

pub(super) fn expand(input: TokenStream) -> TokenStream {
    let mut implementation = parse_macro_input!(input as ItemImpl);
    match injectable(&mut implementation) {
        Ok(generated) => quote!(#implementation #generated).into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn injectable(implementation: &mut ItemImpl) -> syn::Result<proc_macro2::TokenStream> {
    let Some(index) = implementation
        .items
        .iter()
        .position(|item| matches!(item, ImplItem::Fn(method) if method.sig.ident == "new"))
    else {
        return Err(syn::Error::new_spanned(&*implementation, "#[injectable] requires new"));
    };
    let ImplItem::Fn(constructor) = &mut implementation.items[index] else { unreachable!() };

    let mut dependencies = Vec::new();
    for argument in &mut constructor.sig.inputs {
        let FnArg::Typed(argument) = argument else {
            return Err(syn::Error::new_spanned(argument, "new must not have a self receiver"));
        };
        let token = argument
            .attrs
            .iter()
            .position(|attribute| attribute.path().is_ident("inject"))
            .map(|index| argument.attrs.remove(index).parse_args::<Expr>())
            .transpose()?;
        let (ty, optional, lazy) = dependency(&argument.ty)?;
        dependencies.push(match (token, optional, lazy) {
            (Some(token), false, false) => quote!(container.resolve_token::<#ty>(#token)),
            (None, false, false) => quote!(container.resolve::<#ty>()),
            (None, true, false) => quote!(container.try_resolve::<#ty>()),
            (Some(token), true, false) => quote!(container.try_resolve_token::<#ty>(#token)),
            (None, false, true) => quote!(container.lazy::<#ty>()),
            (_, true, true) => {
                return Err(syn::Error::new_spanned(argument, "Lazy cannot be optional"));
            }
            (Some(_), false, true) => {
                return Err(syn::Error::new_spanned(
                    argument,
                    "Lazy token injection is unsupported",
                ));
            }
        });
    }

    let ty = &implementation.self_ty;
    Ok(quote! {
        impl ::arcadia_framework::Injectable for #ty {
            fn create(container: &::arcadia_framework::Container) -> Self {
                Self::new(#(#dependencies),*)
            }
        }
    })
}

fn dependency(ty: &Type) -> syn::Result<(Type, bool, bool)> {
    let Type::Path(path) = ty else {
        return Err(syn::Error::new_spanned(ty, "dependency must have type Arc<T>"));
    };
    let Some(segment) = path.path.segments.last() else { unreachable!() };
    let PathArguments::AngleBracketed(arguments) = &segment.arguments else {
        return Err(syn::Error::new_spanned(ty, "dependency must have type Arc<T>"));
    };
    let optional = segment.ident == "Option";
    let Some(GenericArgument::Type(inner)) = arguments.args.first() else {
        return Err(syn::Error::new_spanned(ty, "dependency must have type Arc<T>"));
    };
    if segment.ident == "Arc" {
        return Ok((inner.clone(), false, false));
    }
    if segment.ident == "Lazy" {
        return Ok((inner.clone(), false, true));
    }
    if !optional {
        return Err(syn::Error::new_spanned(ty, "dependency must have type Arc<T>"));
    }
    let Type::Path(inner_path) = inner else {
        return Err(syn::Error::new_spanned(ty, "dependency must have type Arc<T>"));
    };
    let inner_segment = inner_path.path.segments.last().unwrap();
    let PathArguments::AngleBracketed(inner_arguments) = &inner_segment.arguments else {
        return Err(syn::Error::new_spanned(ty, "dependency must have type Arc<T>"));
    };
    match inner_arguments.args.first() {
        Some(GenericArgument::Type(dependency)) if inner_segment.ident == "Arc" => {
            Ok((dependency.clone(), optional, false))
        }
        _ => Err(syn::Error::new_spanned(ty, "dependency must have type Arc<T>")),
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_str;

    use super::dependency;

    #[test]
    fn parses_required_optional_and_lazy_dependencies() {
        for (source, optional, lazy) in [
            ("Arc<Service>", false, false),
            ("Option<Arc<Service>>", true, false),
            ("Lazy<Service>", false, true),
        ] {
            let (_, actual_optional, actual_lazy) =
                dependency(&parse_str(source).unwrap()).unwrap();
            assert_eq!((actual_optional, actual_lazy), (optional, lazy));
        }
    }

    #[test]
    fn rejects_unsupported_dependency_shapes() {
        for source in ["Service", "Option<Service>"] {
            assert!(dependency(&parse_str(source).unwrap()).is_err());
        }
        let invalid: syn::Result<syn::Type> = parse_str("Arc<");
        assert!(invalid.is_err());
    }
}
