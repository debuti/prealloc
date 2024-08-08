#[cfg(test)]
mod tests {
    use prealloc::prealloc_from_config;

    #[derive(Debug, Default, PartialEq)]
    pub struct TestItem {
        a: u32,
        b: u32,
    }

    #[derive(Debug)]
    pub enum OtherItem {
        L(u32),
        R,
    }

    prealloc_from_config!("prealloc/tests/prealloc_macro/config.json");

    #[test]
    fn test_dispatch_item1() {
        if let Some(item) = dispatch_static!(Item1, TestItem { a: 1, b: 2 }) {
            assert_eq!(item, &mut TestItem { a: 1, b: 2 });
        } else {
            panic!("Item1 bucket shouldnt be depleted yet");
        }

        if let Some(item) = dispatch_static!(Item1, TestItem { a: 3, b: 4 }) {
            assert_eq!(item, &mut TestItem { a: 3, b: 4 });
        } else {
            panic!("Item1 bucket shouldnt be depleted yet");
        }

        if let Some(item) = dispatch_static!(Item1, TestItem { a: 5, b: 6 }) {
            assert_eq!(item, &mut TestItem { a: 5, b: 6 });
        } else {
            panic!("Item1 bucket shouldnt be depleted yet");
        }

        if let Some(_) = dispatch_static!(Item1, TestItem { a: 1, b: 2 }) {
            panic!("Item1 bucket should be depleted")
        }
    }

    #[test]
    fn test_dispatch_item2() {
        for _ in 1..=5 {
            if let Some(item) = dispatch_static!(Item2, OtherItem::L(33)) {
                if let OtherItem::L(value) = item {
                    assert_eq!(*value, 33);
                } else {
                    panic!("Incorrect initialization");
                }
            } else {
                panic!("Item2 bucket shouldnt be depleted yet");
            }
        }

        if let Some(_) = dispatch_static!(Item2, OtherItem::R) {
            panic!("Item2 bucket should be depleted")
        }
    }
}
