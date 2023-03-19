use std::str::FromStr;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use std::string::ToString;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};
use syn::{self, parse_macro_input, AttributeArgs, ItemFn, Lit, Meta, NestedMeta};

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

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input tokens into syntax tree
    let args = parse_macro_input!(attr as AttributeArgs);
    let func = parse_macro_input!(item as ItemFn);

    if args.len() != 2 {
        panic!("route attribute required 2 arguments and {} where provided. First argument should be an HTTP method and the second one URL path", args.len());
    }

    // Extract the HTTP method from the attribute arguments
    let Some(NestedMeta::Meta(Meta::Path(path_token))) = args.get(0) else {
        panic!(
            "Expected HttpMethod as the first argument. Valid HTTP method values are {}",
            HttpMethod::stringified_all()
        );
    };
    let mut http_method_token_stream = proc_macro2::TokenStream::new();
    path_token.to_tokens(&mut http_method_token_stream);
    let http_method =
        syn::parse2::<HttpMethod>(http_method_token_stream).unwrap_or_else(|e| panic!("{e}"));

    // Extract the route path from the attribute arguments
    let Some(NestedMeta::Lit(Lit::Str(route_str))) = args.get(1) else {
        panic!("Expected route path string as the second argument");
    };

    let route_mapping = RouteMapping {
        method: http_method,
        path: route_str.value(),
        handler_name: "".into(),
    };
    println!("{route_mapping:#?}");

    TokenStream::from(quote! {
        #func
    })
}
