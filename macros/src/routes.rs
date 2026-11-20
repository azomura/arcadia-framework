use syn::{ImplItemFn, LitStr};

pub(super) fn take_routes(method: &mut ImplItemFn) -> syn::Result<Vec<(String, LitStr)>> {
    let mut routes = Vec::new();
    let mut retained = Vec::new();

    for attribute in std::mem::take(&mut method.attrs) {
        let verb = attribute.path().get_ident().map(ToString::to_string);

        if let Some(verb) = verb.filter(|verb| {
            matches!(
                verb.as_str(),
                "get" | "post" | "put" | "patch" | "delete" | "head" | "options"
            )
        }) {
            routes.push((verb, attribute.parse_args::<LitStr>()?));
        } else {
            retained.push(attribute);
        }
    }
    method.attrs = retained;

    Ok(routes)
}
