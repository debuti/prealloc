use prealloc::prealloc_from_config;

prealloc_from_config!("res/config.json");

#[derive(Debug, PartialEq)]
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

impl Drop for OtherItem {
    fn drop(&mut self) {
        println!("Dropping OtherItem at {self:p}");
    }
}

fn main() {
    const ITEM1_INITIALIZER: MyItem = MyItem { a: 1, b: 2 };
    assert_eq!(
        dispatch_static!(Item1, ITEM1_INITIALIZER),
        Some(&mut MyItem { a: 1, b: 2 })
    );
    assert_eq!(
        dispatch_static!(Item1, ITEM1_INITIALIZER),
        Some(&mut MyItem { a: 1, b: 2 })
    );
    assert_eq!(
        dispatch_static!(Item1, ITEM1_INITIALIZER),
        Some(&mut MyItem { a: 1, b: 2 })
    );
    assert_eq!(dispatch_static!(Item1, ITEM1_INITIALIZER), None);

    for idx in 0..51 {
        if let Some(item) = dispatch_static!(Item2, OtherItem::L(33)) {
            println!("Retrieved {idx}: {item:p}");
        } else {
            println!("Item2 already dispatched");
        }
    }
}
