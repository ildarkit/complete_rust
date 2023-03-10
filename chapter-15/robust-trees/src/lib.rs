pub mod rb_tree;

#[cfg(test)]
mod tests {
    use super::rb_tree::*;
    use rand::prelude::*;
    use rand::distributions::Uniform;
    use log::debug;

    const VALUES_COUNT: i32 = 20000;

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
    fn walk_random_inserted_values_from_range_test() {
        init_logger();
        let mut rb_tree = RedBlackTree::new();
        let mut rng = thread_rng();
        let mut values: Vec<i32> = (-(VALUES_COUNT / 2)..=VALUES_COUNT / 2).collect();
        let sorted_values: Vec<i32> = values.clone();
        let mut result: Vec<i32> = Vec::with_capacity(VALUES_COUNT.try_into().unwrap());

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

    #[test]
    #[ignore = "walking rbtree stress test"]
    fn walk_random_values_in_range_stress_test() {
        init_logger();
        let mut rng = thread_rng();
        let values_range = Uniform::new_inclusive(-(VALUES_COUNT / 2), VALUES_COUNT / 2);
        let mut result: Vec<i32> = Vec::with_capacity(VALUES_COUNT.try_into().unwrap());
        for _ in 1..=1000 {
            let mut rb_tree = RedBlackTree::new();
            let values: Vec<i32> = values_range
                .sample_iter(&mut rng)
                .take(VALUES_COUNT.try_into().unwrap())
                .collect();
            debug!("values = {:?}", values);
            let mut expected = values.clone();
            expected.sort();
            debug!("inserting values...");
            for i in values.iter() {
                debug!("\nvalue = {:?}", i);
                rb_tree.insert(*i);
            }
            debug!("Walking rbtree...");
            rb_tree.walk_in_order(|node| {
                result.push(node.key);
                debug!("\n {:#?}", node);
            });
            assert_eq!(expected, result, "{}", format!("values = {:?}", values));
            result.clear();
        }
    }

    #[test]
    fn walk_random_values_in_range_test() {
        init_logger(); 
        let mut rb_tree = RedBlackTree::new();
        let mut rng = thread_rng();
        let values_range = Uniform::new_inclusive(-(VALUES_COUNT / 2), VALUES_COUNT / 2);
        let mut result: Vec<i32> = Vec::with_capacity(VALUES_COUNT.try_into().unwrap());
        let values: Vec<i32> = values_range
            .sample_iter(&mut rng)
            .take(VALUES_COUNT.try_into().unwrap())
            .collect();
        let mut expected = values.clone();
        expected.sort();
        debug!("inserting values...");
        for i in values.iter() {
            debug!("\nvalue = {:?}", i);
            rb_tree.insert(*i);
        }
        debug!("Walking rbtree...");
        rb_tree.walk_in_order(|node| {
            result.push(node.key);
            debug!("\n {:#?}", node);
        });
        assert_eq!(expected, result, "{}", format!("values = {:?}", values));
    }

    #[test]
    fn delete_test() {
        init_logger();
        let mut rb_tree = RedBlackTree::new();
        let values = vec![5, 1, 7, 3, 6, 2, 4];
        let expected = vec![1, 3, 4, 5, 6, 7];
        let mut result: Vec<i32> = Vec::with_capacity(6);

        debug!("inserting values...");
        for i in values.iter() {
            debug!("\nvalue = {:?}", i);
            rb_tree.insert(*i);
        }
        debug!("delete value...");
        rb_tree.delete(2);
        debug!("walking rbtree...");
        rb_tree.walk_in_order(|node| {
            result.push(node.key);
            debug!("\n {:#?}", node);
        });
        assert_eq!(result, expected);
    }
}
