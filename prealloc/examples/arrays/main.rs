use prealloc::prealloc_from_config;

prealloc_from_config!("prealloc/examples/arrays/config.json");

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub struct MyItem {
    a: u32,
    b: u32,
}

type MyItemCollection = [MyItem; 3];

fn main() {
    const MYITEM_INITIALIZER: MyItem = MyItem { a: 1, b: 2 };
    if let Some(arr) = dispatch_static!(Multiple, [MYITEM_INITIALIZER; 3]) {
        arr[1].b = 0;
        for o in arr {
            println!("* Item: {o:?}");
        }
    }
}
