mod attr_args;

use quote::quote;
use syn::{spanned::Spanned as _, Token};

#[derive(Debug)]
struct Args {
    vars: Vec<syn::Meta>,
}

impl syn::parse::Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let vars = syn::punctuated::Punctuated::<syn::Meta, Token![,]>::parse_terminated(input)?;
        Ok(
            Args {
                vars: vars.into_iter().collect(),
            }
        )
    }
}

#[proc_macro_attribute]
pub fn command(args: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut function = syn::parse_macro_input!(item as syn::ItemFn);
    // Make sure that the function is async
    if function.sig.asyncness.is_none() {
        let err = syn::Error::new(function.sig.span(), "A command must be async.").into_compile_error();
        return quote!(
            #err
        ).into();
    }

    let attrs = syn::parse_macro_input!(args as Args);
    let description = {
        if let Some(str) = attr_args::get_description(&attrs.vars) {
            quote!(std::option::Option::Some(std::string::String::from(#str)))
        } else {
            quote!(std::option::Option::None)
        }
    };

    // Replace the function name with inner s.t. the top function takes the name
    let command_name = std::mem::replace(&mut function.sig.ident, syn::parse_quote!(inner));

    quote::quote!{
        fn #command_name() -> ::command_framework::Command {
            #function 
            ::command_framework::Command {
                name: stringify!(#command_name).to_string(),
                description: #description,
                kind: ::command_framework::CommandType::SlashCommand,
                inner: |ctx| std::boxed::Box::pin(async move {
                    inner(ctx).await
                })
            }
        }
    }.into()
}