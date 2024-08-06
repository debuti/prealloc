#![no_main]
#![no_std]

use libc_print::*;

use prealloc::prealloc_from_config;

prealloc_from_config!("config.json");

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub struct MyItem {
    a: u32,
    b: u32,
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_eh_personality() {}


#[no_mangle]
#[allow(non_snake_case)]
fn WinMain() -> ! {
        
    const ITEM1_INITIALIZER: MyItem = MyItem { a: 1, b: 2 };
    assert_eq!(
        dispatch_static!(Item1, ITEM1_INITIALIZER),
        Some(&mut MyItem { a: 1, b: 2 })
    );

    libc_println!("Vector-Item = ");
    
    loop {

    }
}