// use config_macro::load_config;

// load_config!("config.json");

#[derive(Debug)]
#[allow(dead_code)]
pub struct MyItem {
    a: u32,
    b: u32,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum OtherItem {
    L(u32),
    R,
}

fn main() {
    // for item in BUFFERS.iter() {
    //     println!("{:?}", item);
    // }

    if let Some(item) = get_and_init_static!("Item1", MyItem {a: 1, b: 2}) {
        println!("Retrieved: {:?}", item);
    } else {
        println!("Item1 already dispatched");
    }
}

use core::mem::MaybeUninit;
use std::sync::Mutex;

const CRATE_MYITEM_DEFAULT: (bool, MaybeUninit<crate::MyItem>) = (false, MaybeUninit::uninit());
pub static ITEM1_MEMORY: Mutex<[(bool, MaybeUninit<crate::MyItem>); 3usize]> =
    Mutex::new([CRATE_MYITEM_DEFAULT; 3usize]);

#[macro_export]
macro_rules! get_and_init_static {
    ($name:expr, $init:expr) => {
        match ($name) {
            "Item1" => {
                let mut result = None;
                let mut lock = ITEM1_MEMORY.lock().unwrap();
                for item in lock.iter_mut() {
                    if !item.0 {
                        item.0 = true;
                        let reference =  unsafe { &mut *(&mut item.1 as *const _ as *mut MaybeUninit<MyItem>) };
                        reference.write($init);
                        result = Some(unsafe{reference.assume_init_mut()}); 
                        break;
                    }
                }
                result
            },
            _ => None,
        }
    };
}
