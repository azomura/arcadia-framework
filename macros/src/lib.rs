mod controller;
mod injectable;
mod module;
mod routes;

use controller::{expand_controller, options::ControllerOptions};

use proc_macro::TokenStream;
use syn::{ItemImpl, parse_macro_input};

/// Register attributed methods as Axum handlers, optionally under a URL prefix.
#[proc_macro_attribute]
pub fn controller(args: TokenStream, input: TokenStream) -> TokenStream {
    let options = parse_macro_input!(args as ControllerOptions);
    let mut implementation = parse_macro_input!(input as ItemImpl);

    match expand_controller(&options, &mut implementation) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Merge controller fields and imported modules into one router.
#[proc_macro_attribute]
pub fn module(args: TokenStream, input: TokenStream) -> TokenStream {
    module::expand(args, input)
}

/// Create an injectable type from its `new` constructor and `Arc<T>` dependencies.
#[proc_macro_attribute]
pub fn injectable(args: TokenStream, input: TokenStream) -> TokenStream {
    if !args.is_empty() {
        return syn::Error::new(proc_macro2::Span::call_site(), "#[injectable] takes no arguments")
            .to_compile_error()
            .into();
    }
    injectable::expand(input)
}
