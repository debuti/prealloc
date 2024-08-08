//! This is a test that we compile in `no_std` mode and we
//! can export/import various items.
//!
//! This doesn't actually run any tests, it's mostly a compile-time verification
//! that things work.

#![no_std]
#![allow(dead_code)]

use prealloc::prealloc_from_config;

prealloc_from_config!("res/config.json");

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub struct MyStruct {
    a: u32,
    b: u32,
}

pub fn test() {
    const ITEM1_INITIALIZER: MyStruct = MyStruct { a: 1, b: 2 };
    assert_eq!(
        dispatch_static!(Anchor, ITEM1_INITIALIZER),
        Some(&mut MyStruct { a: 1, b: 2 })
    );
}