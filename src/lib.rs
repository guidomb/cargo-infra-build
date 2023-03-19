use std::str::FromStr;

use proc_macro::TokenStream;
use quote::quote;
use std::string::ToString;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};
use syn::{parse::Parse, parse_macro_input, ItemFn, LitStr};

#[derive(Debug)]
struct RouteMapping {
    method: HttpMethod,
    path: String,
    handler_name: String,
}

#[derive(Debug, Display, EnumString, EnumIter)]
#[strum(serialize_all = "UPPERCASE")]
enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Options,
}

impl HttpMethod {
    fn stringified_all() -> String {
        let mut iter = HttpMethod::iter();
        let mut result = String::new();

        if let Some(first) = iter.next() {
            result.push_str(&first.to_string());

            if let Some(second_last) = iter.next_back() {
                for item in iter {
                    result.push_str(", ");
                    result.push_str(&item.to_string());
                }

                result.push_str(" or ");
                result.push_str(&second_last.to_string());
            }
        }

        result
    }
}

impl syn::parse::Parse for HttpMethod {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<syn::Ident>()?;
        let raw_method = ident.to_string().to_uppercase();
        HttpMethod::from_str(&raw_method).map_err(|e| {
            syn::Error::new(
                ident.span(),
                format!(
                    "Invalid HTTP method '{raw_method}'. Valid HTTP method values are {}",
                    HttpMethod::stringified_all()
                ),
            )
        })
    }
}

struct RouteMacroArgs {
    http_method: HttpMethod,
    path: String,
}

impl Parse for RouteMacroArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let http_method: HttpMethod = input.parse()?;
        let _: syn::Token![,] = input.parse().map_err(|e| {
            syn::Error::new(input.span(), format!("{e}. Route attribute requires 2 arguments. Expecting second argument to be the route path. e.g: #[route({}, \"/some/path\")]", http_method.to_string()))
        })?;
        let path_node: LitStr = input.parse()?;

        if !input.is_empty() {
            return Err(syn::Error::new(input.span(), "route attribute required 2 arguments but there still are tokens left to be parsed. First argument should be an HTTP method and the second one URL path"));
        }

        Ok(RouteMacroArgs {
            http_method,
            path: path_node.value(),
        })
    }
}

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let RouteMacroArgs { http_method, path } = parse_macro_input!(attr as RouteMacroArgs);

    // TODO parse item function an validate that only the main function is annotated

    let route_mapping = RouteMapping {
        method: http_method,
        path,
        handler_name: "".into(),
    };

    TokenStream::from(quote! {
        #func
    })
}
