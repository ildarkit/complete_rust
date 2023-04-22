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
        assert!(btree.search(&search_char).is_none());
    }

    #[test]
    fn not_found_node() {
        let mut rng = thread_rng();
        let mut btree = BTree::new(3);
        let mut chars: Vec<char> = ('a'..='z').collect();
        chars.shuffle(&mut rng);
        let search_char = 'K';

        for c in chars.iter() {
            let d = Alphanumeric.sample_string(&mut rng, 7);
            let data = Data::new(*c, &d);
            btree.insert(data)
        }
        assert!(btree.search(&search_char).is_none());
    }

    #[test]
    fn founded_node() {
        init_logger();
        let mut rng = thread_rng();
        let mut btree = BTree::new(3);
        let mut s = "test".to_owned();
        let mut chars: Vec<char> = ('a'..='z').collect();
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
        assert_eq!(btree.search(&search_char),
            Some(&Data{key: search_char, data: s})
        );
    }

    #[test]
    #[ignore = "long searching key test in btree on random input"]
    fn found_node_stress() {
        init_logger();
        let mut rng = thread_rng();
        let mut chars: Vec<char> = ('a'..='z').collect();

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
                assert_eq!(btree.search(&search_char),
                    Some(&Data{key: search_char, data: s})
                );
            }
        }
    }

    #[test]
    fn btree_walk_in_order() {
        let mut rng = thread_rng();
        let mut btree = BTree::new(3);
        let mut chars: Vec<char> = ('a'..='z').collect();
        let origin = chars.clone();
        let mut result: Vec<char> = Vec::with_capacity(chars.capacity());
        chars.shuffle(&mut rng);

        for c in chars.iter() {
            let d = Alphanumeric.sample_string(&mut rng, 7);
            let data = Data::new(*c, &d);
            btree.insert(data)
        }
        btree.walk(|data| result.push(data.key));
        assert_eq!(origin, result);
    }

    #[test]
    fn delete_node() {
        let mut rng = thread_rng();
        let mut btree = BTree::new(3);
        let mut chars: Vec<char> = ('a'..='z').collect();
        let deleted = chars[rng.gen_range(0..chars.len())];
        chars.shuffle(&mut rng);

        for c in chars.iter() {
            let d = Alphanumeric.sample_string(&mut rng, 7);
            let data = Data::new(*c, &d);
            btree.insert(data)
        }
        match btree.delete(&deleted) {
            Some(Data{key, ..}) => assert_eq!(key, deleted),
            None => assert!(false, "node with key {deleted} not deleted"),
        }
    }

    #[test]
    #[ignore = "delete key test in btree on random input"]
    fn delete_node_stress() {
        init_logger();
        let mut rng = thread_rng();
        let mut chars: Vec<char> = ('a'..='z').collect();

        for i in 5..chars.len() {
            for _ in 0..10000 {
                let mut btree = BTree::new(3);
                chars.shuffle(&mut rng);
                let test_slice = &chars[..i];
                let deleted = chars[rng.gen_range(0..i)]; 
                for c in test_slice {
                    let d = Alphanumeric.sample_string(&mut rng, 7); 
                    let data = Data::new(*c, &d);
                    btree.insert(data)
                }
                debug!("\nchars = {:?}", test_slice);
                debug!("\ndeleted key = {}", deleted);
                match btree.delete(&deleted) {
                    Some(Data{key, ..}) => assert_eq!(key, deleted),
                    None => assert!(false, "node with key {deleted} not deleted"),
                }
            }
        }
    }
}
