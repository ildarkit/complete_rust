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

    pub fn is_full(&self, order: usize) -> bool {
        self.key.len() == 2 * order - 1
    }

    pub fn keys(&self) -> &[T] {
        &self.key[..]
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

    pub fn get_key(&self, pos: usize) -> Option<&T> {
        match self.key.len() > pos {
            true => Some(&self.key[pos]),
            false => None,
        }
    }

    pub fn remove_key(&mut self, pos: usize) -> Option<T> {
        match self.key.len() > pos {
            true => Some(self.key.remove(pos)),
            false => None,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.node_type == NodeType::Leaf
    }

    fn key_is_equal(&self, pos: usize, key: U) -> bool {
        self.key[pos].key() == key
    }

    pub(crate) fn search(&self, key: U) -> Option<&T> {
        let pos = self.key.iter().rev()
            .position(|k| k.key() <= key)
            .map(|p| self.key.len() - 1 - p);
        debug!("\nsearch pos = {:?}", pos);
        match pos {
            Some(i) => {
                match self.key_is_equal(i, key) {
                    true => self.get_key(i),
                    false => {
                        self.children[i + 1].as_ref().unwrap()
                            .search(key)
                    }
                }
            }
            None if self.is_leaf() => None,
            None => self.children[0].as_ref().unwrap().search(key),
        }
    }

    pub(crate) fn split_child(&mut self, i: usize, order: usize) {
        let mut child = self.children[i].take().unwrap();
        let mut sibling = Node::new(child.get_type());
        
        sibling.key = child.key.split_off(order);
        self.key.insert(i, child.key.remove(order - 1));
        if !child.is_leaf() {
            sibling.children = child.children.split_off(order);
        }
        self.children[i].replace(child);
        self.children.insert(i + 1, Some(sibling));
    }

    pub(crate) fn insert_nonfull(&mut self, value: T, order: usize) {
        let mut pos = self.key.iter().rev()
            .position(|k| k.key() < value.key())
            .map_or(0, |p| self.key.len() - p);
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

    pub(crate) fn delete(&mut self, value: &U, _order: &usize) -> Option<T> {
        let pos = self.key.iter()
            .position(|k| k.key() == *value);
        match pos {
            Some(i) => {
                match self.is_leaf() {
                    true => self.remove_key(i),
                    false => unimplemented!(), 
                }
            }
            None => {
                unimplemented!()
            }
        }
    }
}
