use std::fmt;
use log::debug;
use crate::node::{Tree, Node, Key};

#[derive(Default)]
pub struct BTree<U, T> {
    root: Option<Tree<U, T>>,
    order: usize,
    length: usize,
}

impl<U, T> BTree<U, T>
    where
        T: Clone + Default + fmt::Debug + Key<U>,
        U: Copy + Default + fmt::Debug + PartialEq + PartialOrd,
{
    pub fn new(order: usize) -> Self {
        Self {
            root: Some(Node::new_leaf()),
            order,
            ..Default::default()
        }
    }

    pub fn search(&self, key: U) -> Option<&T> {
        match self.root {
            Some(ref root) => root.search(key),
            None => unreachable!(),
        }
    }

    pub fn insert(&mut self, value: T) {
        self.root = match self.root.take() {
            Some(mut root) => {
                debug!("\nroot = {:#?}", root);
                if root.is_full(self.order) {
                    debug!("\nnode is full");
                    let mut node = Node::new_regular();
                    node.add_child(root);
                    node.split_child(0, self.order);
                    node.insert_nonfull(value, self.order);
                    self.inc_length();
                    Some(node)
                } else {
                    root.insert_nonfull(value, self.order);
                    Some(root)
                }
            }
            None => unreachable!(),
        };
        debug!("\nroot after insert = {:#?}", self.root);
    }

    fn inc_length(&mut self) {
        self.length += 1;
    }
}
