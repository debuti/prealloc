Prealloc allows access to static memory as if it were a heap

# Quickstart

## Installation

Add this dependency to your `Cargo.toml`:

```toml
[dependencies]
prealloc = "0.1.0"
```

## Configuration File

Create a JSON configuration file (ex. `config.json`) that defines the items, their types, and counts:

```json
[
    {
        "name": "MyStruct",
        "type": "my_project::MyStruct",
        "size": 3
    },
    {
        "name": "MyEnumUseCaseA",
        "type": "my_project::MyEnum",
        "size": 5
    },
    {
        "name": "MyEnumUseCaseB",
        "type": "my_project::MyEnum",
        "size": 2
    }
]
```

## Example

```rust
use prealloc::prealloc_from_config;

prealloc_from_config!("path/to/config.json");

#[derive(Debug)]
#[allow(dead_code)]
pub enum MyEnum {
    L(u32),
    R,
}

impl Drop for MyEnum {
    fn drop(&mut self) {
        println!("Dropping MyEnum located at {self:p}");
    }
}

fn main() {
    for idx in 0..=5 {
        // The dispatch_static macro retrieves one preallocated element and initializes it. 
        if let Some(item) = dispatch_static!(MyEnumUseCaseA, MyEnum::L(33)) {
            println!("Retrieved {idx}: {item:p}");
        } else {
            println!("MyEnumUseCaseA depleted");
        }
    }
}

```

Discover more examples with `cargo run --example` on this crate.

# License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

See the LICENSE-APACHE and LICENSE-MIT files for details.

# TODO

* [x] Fetch an static by name and access it
* [x] Provide types for the static memory items
* [N] Don't use statics, instead use a macro in the main fn that creates the memory in the stack. Maybe this is not a good idea (Stack overflow)
* [x] no_std feature flag
* [x] tests (copy from paste crate)
* [] docs (copy from paste crate) and README.md
* [] impl a simple heapless test app with it 
* [] examples: complex type, heap depletion, send and sync
* [] publish crates.io