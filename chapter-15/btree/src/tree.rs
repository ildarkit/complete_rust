use std::{cmp, fmt};
use log::debug;
use crate::node::{Tree, NodeType, Data, Node, Key};

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
        Self { order, ..Default::default() }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn add(&mut self, value: T) {
        let node = match self.root.take() {
            Some(node) => node,
            None => Box::new(Node::new_leaf())
        };
        let (root, _) = self.add_recursive(node, value, true);
        self.root = Some(root);
    }

    fn add_recursive(&mut self, node: Tree<U, T>, value: T, is_root: bool)
        -> (Tree<U, T>, Option<Data<U, T>>)
    {
        let mut node = node;
        let key = value.key();

        match node.get_type() {
            NodeType::Leaf => {
                node.add_key(key, (Some(value), None));
                self.inc_length();
            }
            NodeType::Regular => {
                let (key, (val, tree)) = node.remove_key(key).unwrap();
                let new = self.add_recursive(tree.unwrap(), value, false);
                if val.is_none() {
                    node.add_left_child(Some(new.0));
                } else {
                    node.add_key(key, (val, Some(new.0)));
                }
                if let Some(split_result) = new.1 {
                    let new_key = &split_result.0.clone().unwrap();
                    node.add_key(new_key.key(), split_result);
                }
            }
        }

        if node.len() > self.order {
            let (new_parent, sibling) = node.split();
            if is_root {
                let mut parent = Node::new_regular();
                parent.add_left_child(Some(node));
                parent.add_key(new_parent.key(), (Some(new_parent), Some(sibling)));
                (Box::new(parent), None)
            } else {
                (node, Some((Some(new_parent), Some(sibling))))
            }
        } else {
            (node, None)
        }
    }

    pub fn is_valid(&self) -> bool {
        self.root.as_ref().map_or(false, |tree| {
            let total = self.validate(tree, 0);
            total.0 && total.1 == total.2
        })
    }

    fn validate(&self, node: &Tree<U, T>, level: usize)
        -> (bool, usize, usize)
    {
        match node.get_type() {
            NodeType::Leaf => (node.len() <= self.order, level, level),
            NodeType::Regular => {
                let min_children = if level > 0 {
                    self.order / 2usize
                } else { 2 };
                let key_rules = node.len() <= self.order &&
                    node.len() >= min_children;
                let mut total = (key_rules, usize::max_value(), level);
                for n in node.children().iter().chain(vec![node.left_child()]) {
                    if let Some(ref tree) = n {
                        let stats = self.validate(tree, level + 1);
                        total = (
                            total.0 && stats.0,
                            cmp::min(stats.1, total.1),
                            cmp::max(stats.2, total.2),
                        );
                    }
                }
                total
            }
        }
    }

    pub fn find(&self, key: U) -> Option<T> {
        match self.root.as_ref() {
            Some(tree) => self.find_reqursive(tree, key),
            _ => None,
        }
    }

    fn find_reqursive(&self, node: &Tree<U, T>, key: U) -> Option<T> {
        match node.get_value(key) {
            Some(value) => Some(value.clone()),
            None if node.get_type() != NodeType::Leaf => {
                if let Some(tree) = node.get_child(key) {
                    self.find_reqursive(tree, key)
                } else { None }
            }
            _ => None,
        }
    }

    pub fn walk(&self, mut callback: impl FnMut(&T) -> ()) {
        if let Some(ref root) = self.root {
            self.walk_in_order(root, &mut callback);
        }
    }

    fn walk_in_order(&self, node: &Tree<U, T>, callback: &mut impl FnMut(&T) -> ()) {
        debug!("\nwalk node = {:#?}", node);
        if let Some(ref left) = node.left_child() {
            debug!("\nleft child = {:#?}", left);
            self.walk_in_order(left, callback);
        }
        for i in 0..node.values_len() {
            if let Some(ref value) = node.values()[i] {
                callback(value);
            }
            if let Some(ref child) = node.children()[i] {
                debug!("\nchild node = {:#?}", child);
                self.walk_in_order(&child, callback);
            }
        }
    }

    fn inc_length(&mut self) {
        self.length += 1;
    }
}
