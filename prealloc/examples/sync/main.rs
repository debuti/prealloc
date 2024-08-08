use std::thread;

prealloc::prealloc_from_config!("prealloc/examples/sync/config.json");

type MyType = u8;

fn main() {
    let myvar = dispatch_static!(Anchor, 0).expect("Depleted prealloc storage");

    thread::scope(|s| {
        s.spawn(|| {
            *myvar += 10;
        });
    });

    *myvar += 10;

    println!("{myvar}");
}
