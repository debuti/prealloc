// use config_macro::load_config;

// load_config!("config.json");
use paste::paste;


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
    if let Some(item) = dispatch_static!(Item1, MyItem { a: 1, b: 2 }) {
        println!("Retrieved: {:?}", item);
    } else {
        println!("Item1 already dispatched");
    }

    if let Some(item) = dispatch_static!(Item2, OtherItem::L(33)) {
        println!("Retrieved: {:?}", item);
    } else {
        println!("Item2 already dispatched");
    }
}

use core::mem::MaybeUninit;
use std::sync::Mutex;

const CRATE_MYITEM_DEFAULT: (bool, MaybeUninit<crate::MyItem>) = (false, MaybeUninit::uninit());
pub static ITEM1_MEMORY: Mutex<[(bool, MaybeUninit<crate::MyItem>); 3usize]> =
    Mutex::new([CRATE_MYITEM_DEFAULT; 3usize]);

const CRATE_OTHERITEM_DEFAULT: (bool, MaybeUninit<crate::OtherItem>) =
    (false, MaybeUninit::uninit());
pub static ITEM2_MEMORY: Mutex<[(bool, MaybeUninit<crate::OtherItem>); 3usize]> =
    Mutex::new([CRATE_OTHERITEM_DEFAULT; 3usize]);

fn dispatch_item1(init: MyItem) -> Option<&'static mut MyItem> {
    let mut lock = ITEM1_MEMORY.lock().unwrap();
    for item in lock.iter_mut() {
        if !item.0 {
            item.0 = true;
            let reference =
                unsafe { &mut *(&mut item.1 as *const _ as *mut std::mem::MaybeUninit<MyItem>) };
            return Some(reference.write(init));
        }
    }
    None
}

fn dispatch_item2(init: OtherItem) -> Option<&'static mut OtherItem> {
    let mut lock = ITEM2_MEMORY.lock().unwrap();
    for item in lock.iter_mut() {
        if !item.0 {
            item.0 = true;
            let reference =
                unsafe { &mut *(&mut item.1 as *const _ as *mut std::mem::MaybeUninit<OtherItem>) };
            return Some(reference.write(init));
        }
    }
    None
}

#[macro_export]
macro_rules! dispatch_static {
    ($name:ident, $init:expr) => {
        paste! {[<dispatch _ $name:lower>]($init)}
    };
}
