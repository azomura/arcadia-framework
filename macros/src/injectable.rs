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
        let ty = dependency(&argument.ty)?;
        dependencies.push(match token {
            Some(token) => quote!(container.resolve_token::<#ty>(#token)),
            None => quote!(container.resolve::<#ty>()),
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

fn dependency(ty: &Type) -> syn::Result<&Type> {
    let Type::Path(path) = ty else {
        return Err(syn::Error::new_spanned(ty, "dependency must have type Arc<T>"));
    };
    let Some(segment) = path.path.segments.last() else { unreachable!() };
    let PathArguments::AngleBracketed(arguments) = &segment.arguments else {
        return Err(syn::Error::new_spanned(ty, "dependency must have type Arc<T>"));
    };
    match arguments.args.first() {
        Some(GenericArgument::Type(dependency)) if segment.ident == "Arc" => Ok(dependency),
        _ => Err(syn::Error::new_spanned(ty, "dependency must have type Arc<T>")),
    }
}
