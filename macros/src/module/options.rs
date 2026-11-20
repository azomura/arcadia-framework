use syn::{
    Expr, Token, Type, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

pub(super) struct Binding {
    pub(super) token: Expr,
    pub(super) value: Expr,
}

impl Parse for Binding {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let token = input.parse()?;
        input.parse::<Token![=>]>()?;
        Ok(Self { token, value: input.parse()? })
    }
}

#[derive(Default)]
pub(super) struct ModuleOptions {
    pub(super) controllers: Vec<Type>,
    pub(super) imports: Vec<Type>,
    pub(super) providers: Vec<Type>,
    pub(super) exports: Vec<Type>,
    pub(super) requests: Vec<Type>,
    pub(super) transients: Vec<Type>,
    pub(super) values: Vec<Binding>,
    pub(super) factories: Vec<Binding>,
    pub(super) request_factories: Vec<Binding>,
    pub(super) transient_factories: Vec<Binding>,
    pub(super) token_exports: Vec<Expr>,
    pub(super) lifecycle: Vec<Type>,
    pub(super) globals: Vec<Type>,
    pub(super) dynamic: Vec<Expr>,
}

impl Parse for ModuleOptions {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let mut options = Self::default();
        while !input.is_empty() {
            let name = input.parse::<syn::Ident>()?.to_string();
            let content;
            parenthesized!(content in input);
            match name.as_str() {
                "controllers" => types(&content, &mut options.controllers)?,
                "exports" => types(&content, &mut options.exports)?,
                "imports" => types(&content, &mut options.imports)?,
                "providers" => types(&content, &mut options.providers)?,
                "request" => types(&content, &mut options.requests)?,
                "transient" => types(&content, &mut options.transients)?,
                "values" => bindings(&content, &mut options.values)?,
                "factories" => bindings(&content, &mut options.factories)?,
                "request_factories" => bindings(&content, &mut options.request_factories)?,
                "transient_factories" => bindings(&content, &mut options.transient_factories)?,
                "token_exports" => expressions(&content, &mut options.token_exports)?,
                "lifecycle" => types(&content, &mut options.lifecycle)?,
                "globals" => types(&content, &mut options.globals)?,
                "dynamic" => expressions(&content, &mut options.dynamic)?,
                _ => return Err(syn::Error::new(input.span(), "unsupported module option")),
            }
            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(options)
    }
}

fn types(input: ParseStream<'_>, target: &mut Vec<Type>) -> syn::Result<()> {
    target.extend(Punctuated::<Type, Token![,]>::parse_terminated(input)?);
    Ok(())
}

fn bindings(input: ParseStream<'_>, target: &mut Vec<Binding>) -> syn::Result<()> {
    target.extend(Punctuated::<Binding, Token![,]>::parse_terminated(input)?);
    Ok(())
}

fn expressions(input: ParseStream<'_>, target: &mut Vec<Expr>) -> syn::Result<()> {
    target.extend(Punctuated::<Expr, Token![,]>::parse_terminated(input)?);
    Ok(())
}
