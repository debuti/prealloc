// main_project/src/main.rs
use config_macro::load_config;

load_config!("config.json");

fn main() {
    for item in BUFFERS.iter() {
        println!("{:?}", item);
    }
}
