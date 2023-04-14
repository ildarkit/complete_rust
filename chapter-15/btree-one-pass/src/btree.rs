use std::fmt;
use log::debug;
use crate::node::{Tree, NodeType, NodePosition, Node, Key};

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
            root: Some(Node::new(NodeType::Leaf)),
            order,
            ..Default::default()
        }
    }

    pub fn search(&self, key: U) -> Option<NodePosition<U, T>> {
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
                    let mut node = Node::new(
                        NodeType::Regular
                    );
                    node.add_child(root);
                    node.split_child(1, self.order);
                    node.insert_nonfull(value, self.order);
                    Some(node)
                } else {
                    root.insert_nonfull(value, self.order);
                    Some(root)
                }
            }
            None => { unreachable!() },
        };
        self.inc_length();
    }

    fn inc_length(&mut self) {
        self.length += 1;
    }
}
