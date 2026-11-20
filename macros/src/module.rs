use proc_macro::TokenStream;

use quote::quote;
use syn::{Fields, ItemStruct, parse_macro_input};

mod options;
mod providers;

use options::ModuleOptions;

pub(super) fn expand(args: TokenStream, input: TokenStream) -> TokenStream {
    let options = parse_macro_input!(args as ModuleOptions);
    let mut item = parse_macro_input!(input as ItemStruct);
    match expand_item(&options, &mut item) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn expand_item(
    options: &ModuleOptions, item: &mut ItemStruct,
) -> syn::Result<proc_macro2::TokenStream> {
    let Fields::Named(fields) = &mut item.fields else {
        return Err(syn::Error::new_spanned(item, "#[module] requires a struct with named fields"));
    };
    if !item.generics.params.is_empty() {
        return Err(syn::Error::new_spanned(item, "#[module] does not support generic structs"));
    }

    let mut registrations = Vec::new();
    let mut documents = Vec::new();
    for field in &mut fields.named {
        let imported = field.attrs.iter().any(|attribute| attribute.path().is_ident("import"));
        field.attrs.retain(|attribute| !attribute.path().is_ident("import"));
        let name = &field.ident;
        let ty = &field.ty;
        if imported {
            registrations.push(quote!(context.import(self.#name);));
            documents.push(quote!(<#ty as ::arcadia_framework::Module>::openapi()));
        } else {
            registrations.push(quote!(context.controller_instance(self.#name);));
            documents.push(quote!(<#ty as ::arcadia_framework::Controller>::openapi()));
        }
    }

    let imports = &options.imports;
    let providers = &options.providers;
    let requests = &options.requests;
    let transients = &options.transients;
    let exports = &options.exports;
    let controllers = &options.controllers;
    registrations
        .extend(imports.iter().map(|ty| quote!(context.import(<#ty as Default>::default());)));
    registrations.extend(providers.iter().map(|ty| quote!(context.provide::<#ty>();)));
    registrations.extend(
        requests
            .iter()
            .map(|ty| quote!(context.provide_scoped::<#ty>(::arcadia_framework::Scope::Request);)),
    );
    registrations.extend(
        transients.iter().map(
            |ty| quote!(context.provide_scoped::<#ty>(::arcadia_framework::Scope::Transient);),
        ),
    );
    registrations.extend(exports.iter().map(|ty| quote!(context.export::<#ty>();)));
    registrations.extend(providers::custom(options));
    registrations.extend(options.lifecycle.iter().map(|ty| quote!(context.lifecycle::<#ty>();)));
    registrations.extend(options.globals.iter().map(|ty| {
        quote! {
            context.import_global(::arcadia_framework::GlobalModule(<#ty as Default>::default()));
        }
    }));
    registrations.extend(options.dynamic.iter().map(|module| quote!(context.import(#module);)));
    registrations.extend(controllers.iter().map(|ty| quote!(context.controller::<#ty>();)));
    documents
        .extend(imports.iter().map(|ty| quote!(<#ty as ::arcadia_framework::Module>::openapi())));
    documents.extend(
        controllers.iter().map(|ty| quote!(<#ty as ::arcadia_framework::Controller>::openapi())),
    );
    let name = &item.ident;
    Ok(quote! {
        #item
        impl ::arcadia_framework::Module for #name {
            fn register(self, context: &mut ::arcadia_framework::ModuleContext) {
                #(#registrations)*
            }
            fn openapi() -> ::utoipa::openapi::OpenApi {
                let mut document = ::utoipa::openapi::OpenApi::default();
                #(document.merge(#documents);)*
                document
            }
            fn exports() -> Vec<::std::any::TypeId> {
                vec![#(::std::any::TypeId::of::<#exports>()),*]
            }
        }
    })
}
