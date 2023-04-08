pub mod node;
pub mod tree;

#[cfg(test)]
mod tests {
    use rand::distributions::{Alphanumeric, DistString};
    use super::*;

    #[derive(Clone, Default)]
    struct Value {
        id: usize,
        #[allow(dead_code)]
        value: String,
    }

    impl Value {
        fn new(id: usize, value: &str) -> Self {
            Self { id, value: value.to_owned() }
        }
    }

    impl node::Identity for Value {
        fn id(&self) -> usize {
            self.id
        }
    }

    #[test]
    fn btree_is_valid() {
        let mut btree = tree::BTree::new(3);
        for i in 0..10 {
            let s: String = Alphanumeric
                .sample_string(&mut rand::thread_rng(), 7);
            let v = Value::new(i, &s);
            btree.add(v);
        }
        assert_eq!(btree.length(), 10);
        assert!(btree.is_valid());
    }
}
