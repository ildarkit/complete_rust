use log::debug;
use std::marker::PhantomData;

pub(crate) type Tree<U, T> = Box<Node<U, T>>;

pub trait Key<U: Copy> {
    fn key(&self) -> U;
}

#[derive(Clone, PartialEq, Debug, Default)]
pub enum NodeType {
    #[default]
    Leaf,
    Regular,
}

#[derive(Clone, Default, Debug)]
pub struct Node<U, T> {
    node_type: NodeType,
    key: Vec<T>,
    children: Vec<Option<Tree<U, T>>>,
    phantom: PhantomData<U>
}

impl<U, T> Node<U, T>
    where
        T: Clone + Default + Key<U>,
        U: Copy + Default + PartialEq + PartialOrd,
{
    fn new(node_type: NodeType) -> Tree<U, T> {
        Box::new(Self{
            node_type,
            ..Default::default() 
        })
    }

    pub fn new_leaf() -> Tree<U, T> {
        Self::new(NodeType::Leaf)
    }

    pub fn new_regular() -> Tree<U, T> {
        Self::new(NodeType::Regular)
    }

    pub fn is_full(&self, order: &usize) -> bool {
        self.key.len() == 2 * *order - 1
    }

    pub fn keys(&self) -> &[T] {
        &self.key[..]
    }

    pub fn key_len(&self) -> usize {
        self.key.len()
    }

    pub fn children(&self) -> &[Option<Tree<U, T>>] {
        &self.children[..]
    }

    pub(crate) fn add_child(&mut self, node: Tree<U, T>) {
        self.children.push(Some(node));
    }

    fn get_type(&self) -> NodeType {
        self.node_type.clone()
    }

    pub fn get_key(&self, pos: &usize) -> Option<&T> {
        match self.key.len() > *pos {
            true => Some(&self.key[*pos]),
            false => None,
        }
    }

    pub fn remove_key(&mut self, pos: &usize) -> Option<T> {
        match self.key.len() > *pos {
            true => Some(self.key.remove(*pos)),
            false => None,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.node_type == NodeType::Leaf
    }

    fn key_is_equal(&self, pos: &usize, key: &U) -> bool {
        self.key[*pos].key() == *key
    }

    pub(crate) fn search(&self, key: &U) -> Option<&T> {
        let pos = Self::key_position(&self.key, &|k: &T| k.key() <= *key);
        debug!("\nsearch pos = {:?}", pos);
        match pos {
            Some(ref i) => {
                match self.key_is_equal(i, key) {
                    true => self.get_key(i),
                    false => {
                        self.children[*i + 1].as_ref().unwrap()
                            .search(key)
                    }
                }
            }
            None if self.is_leaf() => None,
            None => self.children[0].as_ref().unwrap().search(key),
        }
    }

    pub(crate) fn split_child(&mut self, i: usize, order: &usize) {
        let mut child = self.children[i].take().unwrap();
        let mut sibling = Node::new(child.get_type());
        
        sibling.key = child.key.split_off(*order);
        self.key.insert(i, child.key.remove(*order - 1));
        if !child.is_leaf() {
            sibling.children = child.children.split_off(*order);
        }
        self.children[i].replace(child);
        self.children.insert(i + 1, Some(sibling));
    }

    pub(crate) fn insert_nonfull(&mut self, value: T, order: &usize) {
        let mut pos = Self::key_position(&self.key, &|k: &T| k.key() < value.key())
            .map_or(0, |p| p + 1);
        debug!("\ninsert pos = {:?}", pos);
        match self.is_leaf() {
            true => self.key.insert(pos, value),
            false => {
                if self.children[pos].as_ref().unwrap().is_full(order) {
                    self.split_child(pos, order);
                    if value.key() > self.key[pos].key() {
                        pos += 1;
                    }
                }
                self.children[pos].as_mut().unwrap()
                    .insert_nonfull(value, order);
            }
        }
    }

    pub(crate) fn delete(&mut self, value: &U, order: &usize) -> Option<T> { 
        let key_pos = self.key.iter()
            .position(|k| k.key() >= *value);
        match key_pos {
            Some(ref pos) => {
                if *pos == 0 && self.key[*pos].key() > *value {
                    return None;
                }
                self.delete_subtree(value, pos, order)
            }
            None => None,
        }
    }

    fn delete_subtree(&mut self, value: &U, pos: &usize, order: &usize)
        -> Option<T>
    {
         match self.key_is_equal(pos, value) {
            // remove key from leaf
            true if self.is_leaf() => self.remove_key(pos),
            // key in the regular node
            true => {
                let replace = self.remove_child_key(value, pos, order);
                match replace {
                    // at least <order> key have
                    Some(replace) => {
                        self.key[*pos] = replace.clone();
                        Some(replace)
                    }
                    // remove all keys from right child
                    // (include key from parent) to the left child
                    // then remove right child and delete key from left child recursively
                    None => {
                        let deleted = self.remove_key(pos).unwrap();
                        self.children[*pos].as_mut().unwrap().key
                            .push(deleted);
                        let key_pos = self.children[*pos].as_mut().unwrap()
                            .key_len() - 1;
                        let sibling_keys = self.children[*pos + 1].as_ref().unwrap()
                            .key.clone();
                        self.children[*pos].as_mut().unwrap().key
                            .extend_from_slice(&sibling_keys[..]);
                        self.children.remove(pos + 1);
                        self.children[*pos].as_mut().unwrap()
                            .delete_subtree(value, &key_pos, order)
                    }
                }
            }
            false => {
                unimplemented!()
            }
        }
    }

    fn key_position(key: &Vec<T>, callback: &impl Fn(&T) -> bool)
        -> Option<usize>
    {
        key.iter().rposition(callback)
    }

    fn at_least_order(&self, order: &usize) -> bool {
        self.key_len() >= *order
    }

    fn remove_from(&mut self, value: &U, pos: &usize, order: &usize)
        -> Option<T>
    {
        if self.at_least_order(order) {
            self.delete_subtree(value, pos, order)
        } else { None }
    }

    fn remove_child_key(&mut self, value: &U, pos: &usize, order: &usize)
        -> Option<T>
    {
        let mut replace = None;
        for i in 0..=1 {
            let child = self.children[*pos + i].as_mut().unwrap();
            let neighbore = if i == 0 {
                child.key_len() - 1
            } else { 0 };
            replace = child
                .remove_from(value, &neighbore, order);
            if replace.is_some() {
                break;
            }
        }
        replace
    }
}
