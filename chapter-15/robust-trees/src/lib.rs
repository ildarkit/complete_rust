mod rb_tree;

#[cfg(test)]
mod tests {
    use super::rb_tree::*;
    use rand::prelude::*;
    use log::debug;

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn not_found_test() {
        let mut rb_tree = RedBlackTree::new();
        let mut value: i32 = -1;
        rb_tree.find(0, |node| {
            if let Some(n) = node {
                value = n.key;
            }
        });
        assert_eq!(value, -1);
        rb_tree.insert(1);
        rb_tree.find(2, |node| {
            if let Some(n) = node {
                value = n.key;
            }
        });
        assert_eq!(value, -1);
    }

    #[test]
    fn found_test() {
        let mut rb_tree = RedBlackTree::new();
        let mut value: i32 = -1;
        rb_tree.insert(0);
        rb_tree.find(0, |node| {
            if let Some(n) = node {
                value = n.key;
            }
        });
        assert_eq!(value, 0);
        for i in 1..=100 {
            rb_tree.insert(i);
        }
        for i in 0..=100 {
            rb_tree.find(i, |node| {
                if let Some(n) = node {
                    value = n.key;
                }
            });
            assert_eq!(value, i);
        }
    }

    #[test]
    fn walk_sorted_values_test() {
        let mut rb_tree = RedBlackTree::new();
        let mut values: Vec<i32> = Vec::with_capacity(100);
        let mut result: Vec<i32> = Vec::with_capacity(100);
        for i in 1..=100 {
            rb_tree.insert(i);
            values.push(i);
        }
        rb_tree.walk_in_order(|node| result.push(node.key));
        assert_eq!(values, result);
    }

    #[test]
    fn walk_random_values_test() {
        init_logger();
        const VEC_LEN: i32 = 20000;
        let mut rb_tree = RedBlackTree::new();
        let mut rng = thread_rng();
        let mut values: Vec<i32> = (-(VEC_LEN / 2)..=VEC_LEN / 2).collect();
        let sorted_values: Vec<i32> = values.clone();
        let mut result: Vec<i32> = Vec::with_capacity(VEC_LEN.try_into().unwrap());

        values.shuffle(&mut rng);
        debug!("Inserting values into rbtree...");
        for i in values.iter() {
            debug!("\nvalue = {:?}", i);
            rb_tree.insert(*i); 
        }
        debug!("Walking rbtree...");
        rb_tree.walk_in_order(|node| {
            result.push(node.key);
            debug!("\n {:#?}", node);
        });
        assert_eq!(sorted_values, result);
    }
}
