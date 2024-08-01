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

#[derive(Deserialize, Debug)]
struct Config {
    buffers: Vec<ConfigItem>,
}

#[proc_macro]
pub fn load_config(input: TokenStream) -> TokenStream {
    // Read the configuration file
    let input = parse_macro_input!(input as LitStr);
    let config_path = input.value();
    let config_data =
        std::fs::read_to_string(config_path).expect("Unable to read configuration file");
    let config: Config = serde_json::from_str(&config_data).expect("Invalid JSON format");

    // Create an iterator of TokenStreams, one item per item in config
    let buffer_data = config.buffers.iter().map(|item| {
        let name: proc_macro2::TokenStream = format!("{}_{}", &item.name, "memory").parse().unwrap();
        let ty: syn::Type = syn::parse_str(&item.r#type).expect("Invalid type");
        let ty_def: proc_macro2::TokenStream = format!("{}_{}", &item.r#type, "default").replace("::", "_").parse().unwrap();
        let size = item.size;
        let item = quote! {
            const #ty_def: MaybeUninit<#ty> = MaybeUninit::uninit();
            pub static #name: [MaybeUninit<#ty>; #size] = [#ty_def; #size];
        };
        println!("Generated item: {}", item);
        item
    });
    
    // Generate the final source block
    let expanded = quote! {
        use core::mem::MaybeUninit;
        
        #(#buffer_data)*
    };

    println!("Generated code:\n{}", expanded);

    TokenStream::from(expanded)
}
