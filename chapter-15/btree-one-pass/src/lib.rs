pub mod node;
pub mod btree;

#[cfg(test)]
mod tests {
    use log::debug;
    use rand::prelude::*;
    use rand::distributions::{Alphanumeric, DistString};
    use super::node::*;
    use super::btree::*;

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
    fn search_in_empty_btree() {
        let btree: BTree<char, Data<char>> = BTree::new(3); 
        let search_char = 'a';
        assert!(btree.search(search_char).is_none());
    }

    #[test]
    fn not_found_node() {
        let mut rng = thread_rng();
        let mut btree = BTree::new(3);
        let mut chars: Vec<char> = ('a'..'z').collect();
        chars.shuffle(&mut rng);
        let search_char = 'K';

        for c in chars.iter() {
            let d = Alphanumeric.sample_string(&mut rng, 7);
            let data = Data::new(*c, &d);
            btree.insert(data)
        }
        assert!(btree.search(search_char).is_none());
    }

    #[test]
    fn found_node() {
        init_logger();
        let mut rng = thread_rng();
        let mut btree = BTree::new(3);
        let mut s = "test".to_owned();
        let mut chars: Vec<char> = ('a'..'z').collect();
        chars.shuffle(&mut rng);
        let search_char = chars[rng.gen_range(0..chars.len())];

        for c in chars.iter() {
            let d = Alphanumeric.sample_string(&mut rng, 7);
            if *c == search_char {
                s = d.clone();
            }
            let data = Data::new(*c, &d);
            debug!("\ndata = {:?}", data);
            btree.insert(data)
        }
        match btree.search(search_char) {
            Some(NodePosition{node, pos, ..}) => {
                debug!("\npos = {pos}, char = {search_char}, node = {:#?}", node);
                assert_eq!(node.get_key(pos), Some(&Data{key: search_char, data: s}));
            }
            None => assert!(false, "node not found"),
        }
    }

    #[test]
    #[ignore = "long searching key test in btree on random input"]
    fn found_node_stress() {
        init_logger();
        let mut rng = thread_rng();
        let mut chars: Vec<char> = ('a'..'z').collect();

        for i in 5..chars.len() {
            for _ in 0..10000 {
                let mut btree = BTree::new(3);
                let mut s = "test".to_owned();
                chars.shuffle(&mut rng);
                let test_slice = &chars[..i];
                let search_char = chars[rng.gen_range(0..i)];
                for c in test_slice {
                    let d = Alphanumeric.sample_string(&mut rng, 7);
                    if *c == search_char {
                        s = d.clone();
                    }
                    let data = Data::new(*c, &d);
                    debug!("\nvalue = {:?}", data);
                    btree.insert(data)
                }
                match btree.search(search_char) {
                    Some(NodePosition{node, pos, ..}) => {
                        debug!("\npos = {pos}, char = {search_char}, node = {:#?}", node);
                        assert_eq!(node.get_key(pos),
                            Some(&Data{key: search_char, data: s.clone()}),
                            "\nerror: {search_char}, {test_slice:?}"
                        );
                    }
                    None => assert!(false,
                        "\nnot found {search_char} in {test_slice:?}"),
                }
            }
        }
    }
}
