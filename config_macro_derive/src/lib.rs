extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use serde::Deserialize;
use syn::{parse_macro_input, LitStr};

#[derive(Deserialize, Debug)]
struct BufferDef {
    name: String,
    size: usize,
}

#[derive(Deserialize, Debug)]
struct Config {
    buffers: Vec<BufferDef>,
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
        let name = &item.name;
        let name: proc_macro2::TokenStream = name.parse().unwrap();
        let size = item.size;
        let u8_default = u8::default();
        let item = quote! {
            pub static #name: [u8; #size] = [#u8_default; #size];
        };
        println!("Generated item: {}", item);
        item
    });
    let buffers = config.buffers.iter().map(|item| {
        let name_str = &item.name;
        let name: proc_macro2::TokenStream = name_str.parse().unwrap();
        let item = quote! {
            Buffer { name: #name_str, value: &#name }
        };
        println!("Generated item: {}", item);
        item
    });

    // Generate the final source block
    let expanded = quote! {
        #[derive(Debug)]
        pub struct Buffer {
            pub name: &'static str,
            pub value: &'static [u8],
        }

        #(#buffer_data)*

        pub static BUFFERS: &[Buffer] = &[
            #(#buffers),*
        ];
    };

    println!("Generated code:\n{}", expanded);

    TokenStream::from(expanded)
}
