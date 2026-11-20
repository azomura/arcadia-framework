use syn::{Ident, LitStr, Token, Type, parenthesized, parse::Parse, punctuated::Punctuated};

#[derive(Default)]
pub(crate) struct ControllerOptions {
    pub(super) path: String,
    pub(super) version: Option<String>,
    pub(super) tag: Option<LitStr>,
    pub(super) description: Option<LitStr>,
    pub(super) guards: Vec<Type>,
    pub(super) interceptors: Vec<Type>,
    pub(super) pipes: Vec<Type>,
    pub(super) filters: Vec<Type>,
    pub(super) middleware: Vec<Type>,
}

impl ControllerOptions {
    pub(super) fn route_prefix(&self) -> String {
        match &self.version {
            Some(version) => format!("/v{}{}", version.trim_start_matches('v'), self.path),
            None => self.path.clone(),
        }
    }
}

impl Parse for ControllerOptions {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        if input.peek(LitStr) {
            return Ok(Self { path: input.parse::<LitStr>()?.value(), ..Self::default() });
        }

        let mut options = Self::default();

        while !input.is_empty() {
            let name = input.parse::<Ident>()?;
            if matches!(
                name.to_string().as_str(),
                "guards" | "interceptors" | "pipes" | "filters" | "middleware"
            ) {
                let content;
                parenthesized!(content in input);
                let types = Punctuated::<Type, Token![,]>::parse_terminated(&content)?;
                match name.to_string().as_str() {
                    "guards" => options.guards.extend(types),
                    "interceptors" => options.interceptors.extend(types),
                    "pipes" => options.pipes.extend(types),
                    "filters" => options.filters.extend(types),
                    "middleware" => options.middleware.extend(types),
                    _ => unreachable!(),
                }
                if !input.is_empty() {
                    input.parse::<Token![,]>()?;
                }
                continue;
            }
            input.parse::<Token![=]>()?;
            let value = input.parse::<LitStr>()?;

            match name.to_string().as_str() {
                "path" => options.path = value.value(),
                "version" => options.version = Some(value.value()),
                "tag" => options.tag = Some(value),
                "description" => options.description = Some(value),
                _ => return Err(syn::Error::new_spanned(name, "unsupported controller option")),
            }

            if !input.is_empty() {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(options)
    }
}
