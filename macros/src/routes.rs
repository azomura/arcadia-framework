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

#[cfg(test)]
mod tests {
    use syn::parse_str;

    use super::take_routes;

    #[test]
    fn extracts_supported_routes_and_preserves_other_attributes() {
        let mut method: syn::ImplItemFn =
            parse_str(r#"#[get("/users")] #[post("/users")] #[derive(Debug)] fn list(&self) {}"#)
                .unwrap();
        let routes = take_routes(&mut method).unwrap();

        assert_eq!(routes.len(), 2);
        assert_eq!(routes[0].0, "get");
        assert_eq!(routes[0].1.value(), "/users");
        assert_eq!(routes[1].0, "post");
        assert_eq!(method.attrs.len(), 1);
        assert_eq!(method.attrs[0].path().segments.last().unwrap().ident, "derive");
    }

    #[test]
    fn rejects_route_without_path_literal() {
        let mut method: syn::ImplItemFn = parse_str(r#"#[get(42)] fn list(&self) {}"#).unwrap();

        assert!(take_routes(&mut method).is_err());
    }
}
