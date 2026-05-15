use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::format_ident;
use syn::{ItemFn, parse2};

#[derive(FromMeta)]
#[darling(derive_syn_parse)]
struct NativePluginAttrs {
    /// Namespace of the native function, used to group related functions together.
    namespace: String,
    /// Name of the native function, used to identify the function when called from Racket code.
    /// 
    /// If not provided, the function name will be used as the plugin name.
    name: Option<String>,
    /// Arity of the native function, used to check if the correct number of arguments is passed when called from Racket code.
    arity: usize,
    /// Indicates if the function is variadic, meaning it can accept a variable number of arguments.
    /// 
    /// This does not affect arity, and at least the minimum number of arguments must be passed when calling a variadic function.
    #[darling(default)]
    variadic: bool,
}

pub fn native_plugin(attrs: TokenStream, input: TokenStream) -> syn::Result<TokenStream> {
    let function = parse2::<ItemFn>(input)?;
    let attrs = parse2::<NativePluginAttrs>(attrs.into())?;

    let namespace = attrs.namespace;
    let name = attrs.name.unwrap_or_else(|| function.sig.ident.to_string());
    let arity = attrs.arity;
    let variadic = attrs.variadic;
    let function_name = &function.sig.ident;
    let mod_name = format_ident!("{}_{}_plugin", namespace, function_name);

    Ok(quote::quote! {
        #function

        pub mod #mod_name {
            use common::value::GCValue;
            use vm::plugin::{NativePlugin, MaybeGcValue, NativeError};

            pub fn plugin() -> NativePlugin {
                NativePlugin {
                    namespace: #namespace,
                    name: #name,
                    arity: (#arity, #variadic),
                    call: super::#function_name,
                }
            }
        }
    })
}