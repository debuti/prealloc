prealloc::prealloc_from_config!("prealloc/examples/sizes/config.json");

#[derive(Debug, PartialEq)]
pub struct MyStruct {
    a: u32,
    b: u64,
}

fn main() {
    let first_mutable_ref = dispatch_static!(
        Anchor,
        MyStruct {
            a: 0x44556677,
            b: 0xAABBCCDD
        }
    )
    .expect("Depleted prealloc storage");
    let second_mutable_ref =
        dispatch_static!(Anchor, MyStruct { a: 1, b: 2 }).expect("Depleted prealloc storage");

    let delta = (second_mutable_ref as *mut _ as usize) - (first_mutable_ref as *mut _ as usize);

    println!(
        "Size of ty:              {:#0X} B",
        std::mem::size_of::<MyStruct>()
    );
    println!("Separation between refs: {delta:#0X} B");
}
