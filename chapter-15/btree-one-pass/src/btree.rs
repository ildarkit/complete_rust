use std::fmt;
use std::iter::zip;
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

    pub fn search(&self, key: &U) -> Option<&T> {
        match self.root {
            Some(ref root) => root.search(key),
            None => unreachable!(),
        }
    }

    pub fn insert(&mut self, value: T) {
        self.root = match self.root.take() {
            Some(mut root) => {
                debug!("\nroot = {:#?}", root);
                if root.is_full(&self.order) {
                    debug!("\nnode is full");
                    let mut node = Node::new_regular();
                    node.add_child(root);
                    node.split_child(0, &self.order);
                    node.insert_nonfull(value, &self.order);
                    self.inc_length();
                    Some(node)
                } else {
                    root.insert_nonfull(value, &self.order);
                    Some(root)
                }
            }
            None => unreachable!(),
        };
        debug!("\nroot after insert = {:#?}", self.root);
    }

    pub fn delete(&mut self, value: &U) -> Option<T> {
        let deleted = self.root.as_mut().unwrap().delete(value, &self.order);
        self.root = match self.root.take() {
            Some(mut root) => {
                if root.is_empty() && root.children().len() > 0 {
                    root = root.children()[0].clone().unwrap();
                    self.dec_length();
                }
                Some(root)
            }
            None => unreachable!(),
        };
        debug!("\nroot after delete = {:#?}", self.root);
        deleted
    }

    pub fn walk(&self, mut callback: impl FnMut(&T)) {
        Self::walk_in_order(self.root.as_ref().unwrap(), &mut callback);
    }

    fn walk_in_order(node: &Tree<U, T>, callback: &mut impl FnMut(&T)) {
        match node.is_leaf() {
            true => {
                for key in node.keys() {
                    callback(key);
                }
            }
            false => {
                let pair = zip(node.keys(), node.children());
                for (key, child) in pair {
                    Self::walk_in_order(child.as_ref().unwrap(), callback);
                    callback(key);
                }
                if let Some(last_child) = node.children().iter().rev().next() {
                    Self::walk_in_order(
                        last_child.as_ref().unwrap(),
                        callback
                    );
                }
            }
        }
    }

    fn inc_length(&mut self) {
        self.length += 1;
    }

    fn dec_length(&mut self) {
        self.length -= 1;
    }

    pub fn length(&self) -> usize {
        self.length
    }
}
