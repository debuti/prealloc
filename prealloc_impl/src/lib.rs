extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use serde::Deserialize;
use syn::{parse_macro_input, LitStr};

#[derive(Deserialize, Debug)]
struct ConfigItem {
    name: String,
    r#type: String,
    size: usize,
}

#[proc_macro]
pub fn prealloc_from_config(input: TokenStream) -> TokenStream {
    let config: Vec<ConfigItem> = {
        let input = parse_macro_input!(input as LitStr);
        let config_path = input.value();
        let config_data =
            std::fs::read_to_string(config_path).expect("Unable to read configuration file");
        serde_json::from_str(&config_data).expect("Invalid JSON format")
    };

    let memories = config.iter().map(|item| {
        let name: proc_macro2::TokenStream = format!("{}_MEMORY", &item.name.to_uppercase())
            .parse()
            .unwrap();
        let ty: syn::Type = syn::parse_str(&item.r#type).expect("Invalid type");
        let ty_def: proc_macro2::TokenStream = format!("{}_DEFAULT", &item.r#type.to_uppercase())
            .replace("::", "_")
            .parse()
            .unwrap();
        let size = item.size;
        quote! {
            const #ty_def: (bool, MaybeUninit<#ty>) = (false, MaybeUninit::uninit());
            pub static #name: Mutex<[(bool, MaybeUninit<#ty>); #size]> = Mutex::new([#ty_def; #size]);
        }
    });

    let dispatchers = config.iter().map(|item| {
        let name: proc_macro2::TokenStream = format!("{}", &item.name).parse().unwrap();
        let ty: syn::Type = syn::parse_str(&item.r#type).expect("Invalid type");
        quote! {
            _dispatch_impl!{#name, #ty}
        }
    });

    let expanded = quote! {
        use core::mem::MaybeUninit;
        // FIXME: Only if !no_std
        use std::sync::Mutex;
        use prealloc::paste;

        #(#memories)*

        #[macro_export]
        macro_rules! _dispatch_impl {
            ($name:ident, $type:ty) => {
                paste!{
                    fn [<dispatch_ $name:lower>](init: $type) -> Option<&'static mut $type> {
                        let mut lock = [<$name:upper _MEMORY>].lock().unwrap();
                        for item in lock.iter_mut() {
                            if !item.0 {
                                item.0 = true;
                                let reference =
                                    unsafe { &mut *(&mut item.1 as *const _ as *mut std::mem::MaybeUninit<$type>) };
                                return Some(reference.write(init));
                            }
                        }
                        None
                    }
                }
            };
        }

        #(#dispatchers)*

        #[macro_export]
        macro_rules! dispatch_static {
            ($name:ident, $init:expr) => {
                paste!{[<dispatch _ $name:lower>]($init)}
            };
        }

    };

    TokenStream::from(expanded)
}
