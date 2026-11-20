use syn::{
    Expr, ExprLit, ImplItemFn, Lit, LitStr, MetaNameValue, Token, Type, punctuated::Punctuated,
};

#[derive(Default)]
pub(super) struct Layers {
    pub(super) guards: Vec<Type>,
    pub(super) interceptors: Vec<Type>,
    pub(super) pipes: Vec<Type>,
    pub(super) filters: Vec<Type>,
    pub(super) middleware: Vec<Type>,
    pub(super) metadata: Vec<(LitStr, LitStr)>,
}

pub(super) fn take(method: &mut ImplItemFn) -> syn::Result<Layers> {
    let mut layers = Layers::default();
    let mut retained = Vec::new();
    for attribute in std::mem::take(&mut method.attrs) {
        let name = attribute.path().get_ident().map(ToString::to_string);
        match name.as_deref() {
            Some("guards") => types(&attribute, &mut layers.guards)?,
            Some("interceptors") => types(&attribute, &mut layers.interceptors)?,
            Some("pipes") => types(&attribute, &mut layers.pipes)?,
            Some("filters") => types(&attribute, &mut layers.filters)?,
            Some("middleware") => types(&attribute, &mut layers.middleware)?,
            Some("metadata") => layers.metadata.extend(parse_metadata(&attribute)?),
            _ => retained.push(attribute),
        }
    }
    method.attrs = retained;
    Ok(layers)
}

fn types(attribute: &syn::Attribute, target: &mut Vec<Type>) -> syn::Result<()> {
    target.extend(attribute.parse_args_with(Punctuated::<Type, Token![,]>::parse_terminated)?);
    Ok(())
}

fn parse_metadata(attribute: &syn::Attribute) -> syn::Result<Vec<(LitStr, LitStr)>> {
    let parser = Punctuated::<MetaNameValue, Token![,]>::parse_terminated;
    let values = attribute.parse_args_with(parser)?;
    values
        .into_iter()
        .map(|value| {
            let Some(key) = value.path.get_ident() else {
                return Err(syn::Error::new_spanned(
                    value.path,
                    "metadata key must be an identifier",
                ));
            };
            let Expr::Lit(ExprLit { lit: Lit::Str(text), .. }) = value.value else {
                return Err(syn::Error::new_spanned(
                    value.value,
                    "metadata value must be a string",
                ));
            };
            Ok((LitStr::new(&key.to_string(), key.span()), text))
        })
        .collect()
}
