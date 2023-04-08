use std::cmp;
use crate::node::{Tree, NodeType, Data, Node, Identity};

#[derive(Default)]
pub struct BTree<T> {
    root: Option<Tree<T>>,
    order: usize,
    length: usize,
}

impl<T> BTree<T>
    where
        T: Clone + Default + Identity
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

    fn add_recursive(&mut self, node: Tree<T>, value: T, is_root: bool)
        -> (Tree<T>, Option<Data<T>>)
    {
        let mut node = node;
        let id = value.id();

        match node.get_type() {
            NodeType::Leaf => {
                node.add_key(id, (Some(value), None));
                self.inc_length();
            }
            NodeType::Regular => {
                let (key, (val, tree)) = node.remove_key(id).unwrap();
                let new = self.add_recursive(tree.unwrap(), value, false);
                if val.is_none() {
                    node.add_left_child(Some(new.0));
                } else {
                    node.add_key(key, (val, Some(new.0)));
                }
                if let Some(split_result) = new.1 {
                    let new_id = &split_result.0.clone().unwrap();
                    node.add_key(new_id.id(), split_result);
                }
            }
        }

        if node.len() > self.order {
            let (new_parent, sibling) = node.split();
            if is_root {
                let mut parent = Node::new_regular();
                parent.add_left_child(Some(node));
                parent.add_key(new_parent.id(), (Some(new_parent), Some(sibling)));
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

    fn validate(&self, node: &Tree<T>, level: usize)
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

    pub fn find(&self, id: usize) -> Option<T> {
        match self.root.as_ref() {
            Some(tree) => self.find_reqursive(tree, id),
            _ => None,
        }
    }

    fn find_reqursive(&self, node: &Tree<T>, id: usize) -> Option<T> {
        match node.get_value(id) {
            Some(value) => Some(value.clone()),
            None if node.get_type() != NodeType::Leaf => {
                if let Some(tree) = node.get_child(id) {
                    self.find_reqursive(tree, id)
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

    fn walk_in_order(&self, node: &Tree<T>, callback: &mut impl FnMut(&T) -> ()) {
        if let Some(ref left) = node.left_child() {
            self.walk_in_order(left, callback);
        }
        for i in 0..node.values_len() {
            if let Some(ref k) = node.get_value(i) {
                callback(k);
            }
            if let Some(ref c) = node.get_child(i) {
                self.walk_in_order(&c, callback);
            }
        }
    }

    fn inc_length(&mut self) {
        self.length += 1;
    }
}
