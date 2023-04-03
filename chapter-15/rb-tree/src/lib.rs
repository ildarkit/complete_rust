use std::fmt;
use std::hash::Hash;
use std::collections::HashMap;

mod node;
mod repo;
pub mod rb_tree;

pub use crate::node::{Node, Operations};
pub use crate::repo::Repository;

#[derive(Default, Debug, Clone)]
pub struct RepoNode<T, U>(HashMap<U, Node<T, U>>)
    where 
        T: Eq + Hash + Default + fmt::Debug + Clone,
        U: Eq + Hash + Default + fmt::Debug + Copy + PartialOrd;

impl<T, U> Repository<T, U> for RepoNode<T, U>
    where 
        T: Eq + Hash + Default + fmt::Debug + Clone,
        U: Eq + Hash + Default + fmt::Debug + Copy + PartialOrd
{
    type Output = Node<T, U>;

    fn new() -> Self {
        Self(HashMap::new())
    }

    fn add(&mut self, id: &U, key: T) {
        self.0.insert(*id, Node::new(id, key));
    }

    fn remove(&mut self, node_id: &U) -> Option<Self::Output> {
        self.0.remove(node_id)
    }

    fn get(&self, node_id: &U) -> Option<&Self::Output> {
        self.0.get(node_id)
    }

    fn get_mut(&mut self, node_id: &U) -> Option<&mut Self::Output> {
        self.0.get_mut(node_id)
    }
}

#[cfg(test)]
mod tests {
    use super::{RepoNode, Repository, Operations};
    use super::rb_tree::*;
    use rand::prelude::*;
    use rand::distributions::Uniform;
    use log::debug;

    const VALUES_COUNT: i32 = 2000;

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn not_found() {
        let repo: &mut RepoNode<i32, u32> = &mut RepoNode::new();
        let mut rb_tree = RedBlackTree::new();
        let mut value: i32 = -1;
        rb_tree.find(repo, 0, |node| {
            if let Some(n) = node {
                value = n.key();
            }
        });
        assert_eq!(value, -1);
        rb_tree.insert(repo, 1);
        rb_tree.find(repo, 2, |node| {
            if let Some(n) = node {
                value = n.key();
            }
        });
        assert_eq!(value, -1);
    }

    #[test]
    fn found() {
        let repo: &mut RepoNode<i32, u32> = &mut RepoNode::new();
        let mut rb_tree = RedBlackTree::new();
        let mut value: i32 = -1;
        rb_tree.insert(repo, 0);
        rb_tree.find(repo, 0, |node| {
            if let Some(n) = node {
                value = n.key();
            }
        });
        assert_eq!(value, 0);
        for i in 1..=100 {
            rb_tree.insert(repo, i);
        }
        for i in 0..=100 {
            rb_tree.find(repo, i, |node| {
                if let Some(n) = node {
                    value = n.key();
                }
            });
            assert_eq!(value, i);
        }
    }

    #[test]
    fn walk_sorted_values() {
        let mut rb_tree = RedBlackTree::new();
        let repo: &mut RepoNode<i32, u32> = &mut RepoNode::new();
        let mut values: Vec<i32> = Vec::with_capacity(100);
        let mut result: Vec<i32> = Vec::with_capacity(100);
        for i in 1..=100 {
            rb_tree.insert(repo, i);
            values.push(i);
        }
        rb_tree.walk_in_order(repo, |node| result.push(node.key()));
        assert_eq!(values, result);
    }

    #[test]
    fn walk_random_inserted_values_from_range() {
        init_logger();
        let mut rb_tree = RedBlackTree::new();
        let repo: &mut RepoNode<i32, u32> = &mut RepoNode::new();
        let mut rng = thread_rng();
        let mut values: Vec<i32> = (-(VALUES_COUNT / 2)..=VALUES_COUNT / 2).collect();
        let sorted_values: Vec<i32> = values.clone();
        let mut result: Vec<i32> = Vec::with_capacity(VALUES_COUNT.try_into().unwrap());

        values.shuffle(&mut rng);
        debug!("Inserting values into rbtree...");
        for i in values.iter() {
            debug!("\nvalue = {:?}", i);
            rb_tree.insert(repo, *i); 
        }
        debug!("Walking rbtree...");
        rb_tree.walk_in_order(repo, |node| {
            result.push(node.key());
            debug!("\n {:#?}", node);
        });
        assert_eq!(sorted_values, result);
    }

    #[test]
    #[ignore = "walking rbtree stress test"]
    fn walk_random_values_in_range_stress() {
        init_logger();
        let mut rng = thread_rng();
        let values_range = Uniform::new_inclusive(-(VALUES_COUNT / 2), VALUES_COUNT / 2);
        let mut result: Vec<i32> = Vec::with_capacity(VALUES_COUNT.try_into().unwrap());
        for i in 1..=1000 {
            if i % 100 == 0 {
                println!("step = {}", i);
            }
            let mut rb_tree = RedBlackTree::new();
            let repo: &mut RepoNode<i32, u32> = &mut RepoNode::new();
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
                rb_tree.insert(repo, *i);
            }
            debug!("Walking rbtree...");
            rb_tree.walk_in_order(repo, |node| {
                result.push(node.key());
                debug!("\n {:#?}", node);
            });
            assert_eq!(expected, result, "{}", format!("values = {:?}", values));
            result.clear();
        }
    }

    #[test]
    fn walk_random_values_in_range() {
        init_logger(); 
        let mut rb_tree = RedBlackTree::new();
        let repo: &mut RepoNode<i32, u32> = &mut RepoNode::new();
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
            rb_tree.insert(repo, *i);
        }
        debug!("Walking rbtree...");
        rb_tree.walk_in_order(repo, |node| {
            result.push(node.key());
            debug!("\n {:#?}", node);
        });
        assert_eq!(expected, result, "{}", format!("values = {:?}", values));
    } 

    #[test]
    fn delete_random() {
        init_logger();
        let rng = thread_rng();
        let values_range = Uniform::new_inclusive(-(VALUES_COUNT / 2), VALUES_COUNT / 2);
        let mut result: Vec<i32> = Vec::with_capacity(VALUES_COUNT.try_into().unwrap());
        run_delete(rng.clone(), values_range, &mut result);
    }

    #[test]
    #[ignore = "delete random from rbtree stress test"]
    fn delete_random_stress() {
        init_logger();
        let rng = thread_rng();
        let values_range = Uniform::new_inclusive(-(VALUES_COUNT / 2), VALUES_COUNT / 2);
        let mut result: Vec<i32> = Vec::with_capacity(VALUES_COUNT.try_into().unwrap());

        for i in 1..=1000 {
            if i % 100 == 0 {
                println!("step = {}", i);
            }
            run_delete(rng.clone(), values_range, &mut result);
            result.clear();
        }
    }

    fn run_delete(mut rng: ThreadRng, distrib: Uniform<i32>, result: &mut Vec<i32>) {
        let mut rb_tree = RedBlackTree::new();
        let repo: &mut RepoNode<i32, u32> = &mut RepoNode::new();
        let values: Vec<i32> = distrib
            .sample_iter(&mut rng)
            .take(VALUES_COUNT.try_into().unwrap())
            .collect();
        let remove_index = rng.gen_range(0..VALUES_COUNT) as usize;
        let mut expected = values.clone();
        expected.sort();
        debug!("expected before remove = {:?}", expected);
        debug!("index of remove = {:?}", remove_index);
        let deleted = expected.remove(remove_index);
        debug!("deleted value = {}", deleted);
        debug!("expected after remove = {:?}", expected);
        debug!("values = {:?}", values);

        debug!("inserting values...");
        for i in values.iter() {
            debug!("\nvalue = {:?}", i);
            rb_tree.insert(repo, *i);
        }
        debug!("delete value from rbtree...");
        rb_tree.delete(repo, deleted);
        debug!("walking rbtree...");
        rb_tree.walk_in_order(repo, |node| {
            result.push(node.key());
            debug!("\n {:#?}", node);
        });
        debug!("result = {:?}", result);
        assert_eq!(*result, expected, "{}",
            format!("deleted = {}, values = {:?}", deleted, values));
    }

    #[test]
    fn delete_all() {
        init_logger();
        let rng = thread_rng();
        let values_range = Uniform::new_inclusive(-(VALUES_COUNT / 2), VALUES_COUNT / 2);
        let mut result: Vec<i32> = Vec::with_capacity(VALUES_COUNT.try_into().unwrap());

        run_clear_rbtree(rng.clone(), values_range, &mut result);

    }

    #[test]
    #[ignore = "clear rbtree stress test"]
    fn delete_all_stress() {
        init_logger();
        let rng = thread_rng();
        let values_range = Uniform::new_inclusive(-(VALUES_COUNT / 2), VALUES_COUNT / 2);
        let mut result: Vec<i32> = Vec::with_capacity(VALUES_COUNT.try_into().unwrap());

        for i in 1..=1000 {
            if i % 100 == 0 {
                println!("step = {}", i);
            }
            run_clear_rbtree(rng.clone(), values_range, &mut result);
        }
    }

    fn run_clear_rbtree(mut rng: ThreadRng, distrib: Uniform<i32>, result: &mut Vec<i32>) {
        let mut rb_tree = RedBlackTree::new();
        let repo: &mut RepoNode<i32, u32> = &mut RepoNode::new();
        let values: Vec<i32> = distrib
            .sample_iter(&mut rng)
            .take(VALUES_COUNT.try_into().unwrap())
            .collect();
        debug!("values = {:?}", values);
        let mut shuffled = values.clone();
        shuffled.shuffle(&mut rng); 
        debug!("shuffled = {:?}", shuffled);

        debug!("inserting values...");
        for i in values.iter() {
            rb_tree.insert(repo, *i);
        }
        debug!("delete all values from rbtree...");
        while !shuffled.is_empty() {
            let deleted = shuffled.pop().unwrap();
            debug!("delete value = {}", deleted);
            rb_tree.delete(repo, deleted);
        }
        debug!("walking rbtree...");
        rb_tree.walk_in_order(repo, |node| {
            result.push(node.key());
            debug!("\n node = {:#?}", node);
        });
        debug!("result = {:?}", result);
        assert!(result.is_empty());
        assert_eq!(result.len(), rb_tree.len());
    }
}
