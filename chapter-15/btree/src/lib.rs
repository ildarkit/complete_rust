pub mod node;
pub mod tree;

#[cfg(test)]
mod tests {
    use rand::prelude::*;
    use rand::distributions::{Alphanumeric, DistString};
    use super::node::*;
    use super::tree::*;

    const LEN: usize = 10;

    #[derive(Clone, Default, Debug, PartialEq)]
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

    impl Identity for Value {
        fn id(&self) -> usize {
            self.id
        }
    }

    #[test]
    fn btree_is_valid() {
        let mut btree = BTree::new(3);
        for i in 0..LEN {
            let s: String = Alphanumeric
                .sample_string(&mut rand::thread_rng(), 7);
            let v = Value::new(i, &s);
            btree.add(v);
        }
        assert_eq!(btree.length(), LEN);
        assert!(btree.is_valid());
    }

    #[test]
    fn btree_found() {
        let mut rng = rand::thread_rng();
        let mut btree = BTree::new(4);
        let mut index: Vec<usize> = (0..LEN).collect();
        index.shuffle(&mut rng);
        for i in index.iter() {
            let s: String = Alphanumeric
                .sample_string(&mut rng, 7); 
            let v = Value::new(*i, &s);
            btree.add(v);
        }
        index.shuffle(&mut rng);
        for i in index.iter() {
            assert_ne!(btree.find(*i), None);
        }
    }

    #[test]
    fn btree_not_found() {
        let mut rng = rand::thread_rng();
        let mut btree = BTree::new(4);
        let mut index: Vec<usize> = (0..LEN).collect();
        index.shuffle(&mut rng);
        for i in index.iter() {
            let s: String = Alphanumeric
                .sample_string(&mut rng, 7); 
            let v = Value::new(*i, &s);
            btree.add(v);
        }
        index.shuffle(&mut rng);
        for i in index.iter() {
            assert_eq!(btree.find(*i + LEN), None);
        }
    }

}
