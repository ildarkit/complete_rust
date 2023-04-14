pub mod node;
pub mod btree;

#[cfg(test)]
mod tests {
    use log::debug;
    use rand::prelude::*;
    use rand::distributions::{Alphanumeric, DistString};
    use super::node::*;
    use super::btree::*;

    // const LEN: usize = 10000;

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[derive(Clone, Default, Debug, PartialEq)]
    struct Data<U> {
        key: U,
        data: String,
    }

    impl<U> Data<U> {
        fn new(key: U, data: &str) -> Self {
            Self { key, data: data.to_owned() }
        }
    }

    impl<U: Copy> Key<U> for Data<U> {
        fn key(&self) -> U {
            self.key
        }
    }

    #[test]
    fn found_node() {
        init_logger();
        const SEARCH_CHAR: char = 'm';
        let mut rng = thread_rng();
        let mut btree = BTree::new(3);
        let mut s = "test".to_owned();

        for c in "qwertyasdfglkjhzxcvmnbpoiu".chars() {
            let d = Alphanumeric.sample_string(&mut rng, 7);
            if c == SEARCH_CHAR {
                s = d.clone();
            }
            let data = Data::new(c, &d);
            debug!("\nvalue = {:?}", data);
            btree.insert(data)
        }
        match btree.search(SEARCH_CHAR) {
            Some(NodePosition{node, pos, ..}) => {
                debug!("\npos = {pos}, node = {:#?}", node);
                assert_eq!(node.get_key(pos), Some(&Data{key: SEARCH_CHAR, data: s}));
            }
            None => assert!(false, "node not found"),
        }
    }
}
