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
        #[allow(dead_code)]
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
        let mut rng = thread_rng();
        let mut btree = BTree::new(3);

        for c in "string".chars() {
            let data = Data::new(c, &Alphanumeric.sample_string(&mut rng, 7));
            btree.insert(data)
        }
        match btree.search('s') {
            Some(NodePosition{node, pos: i, ..}) => {
                debug!("\npos = {i}, node = {:#?}", node)
            }
            None => assert!(false, "node not found"),
        }
    }
}
