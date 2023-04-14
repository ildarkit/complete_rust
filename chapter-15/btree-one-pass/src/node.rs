use std::marker::PhantomData;

pub(crate) type Tree<U, T> = Box<Node<U, T>>;

pub trait Key<U: Copy> {
    fn key(&self) -> U;
}

#[derive(Debug, Default)]
pub struct NodePosition<U, T> {
    pub node: Tree<U, T>,
    pub pos: usize,
    phantom: PhantomData<U>
}

#[derive(Clone, PartialEq, Debug, Default)]
pub(crate) enum NodeType {
    #[default]
    Leaf,
    Regular,
}

#[derive(Clone, Default, Debug)]
pub struct Node<U, T> {
    node_type: NodeType,
    key: Vec<T>,
    children: Vec<Option<Tree<U, T>>>,
    key_count: usize,
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

    pub(crate) fn is_full(&self, order: usize) -> bool {
        self.key_count() == 2 * order - 1
    }

    pub fn children(&self) -> &[Option<Tree<U, T>>] {
        &self.children[..]
    }

    pub(crate) fn add_child(&mut self, node: Tree<U, T>) {
        self.children.push(Some(node));
    }

    pub(crate) fn get_type(&self) -> NodeType {
        self.node_type.clone()
    }

    pub fn get_key(&self, pos: usize) -> Option<&T> {
        match self.key.len() > pos {
            true => Some(&self.key[pos]),
            false => None,
        }
    }

    pub fn key_count(&self) -> usize {
        self.key_count
    }

    fn set_key_count(&mut self, key_count: usize) {
        self.key_count = key_count;
    }

    fn inc_key_count(&mut self) {
        self.key_count += 1;
    }

    pub fn is_leaf(&self) -> bool {
        self.node_type == NodeType::Leaf
    }

    pub(crate) fn search(&self, key: U) -> Option<NodePosition<U, T>> {
        let pos = self.key.iter().position(|k| k.key() > key);
        match pos {
            Some(i) if self.key[i].key() == key => {
                Some(NodePosition {
                    node: Box::new(self.clone()),
                    pos: i,
                    ..Default::default()
                })
            } 
            Some(_) if self.is_leaf() => None,
            Some(i) => self.children[i].as_ref().unwrap().search(key),
            None => self.children[0].as_ref().unwrap().search(key),
        }
    }

    pub(crate) fn split_child(&mut self, i: usize, order: usize) {
        let mut child = self.children[i].take().unwrap();
        let mut sibling = Node::new(child.get_type());
        sibling.set_key_count(order - 1);
        sibling.key = child.key.split_off(order - 1);
        if !child.is_leaf() {
            sibling.children = child.children.split_off(order);
        }
        child.set_key_count(order - 1);
        self.key.insert(i, child.key[order].clone());
        self.children[i].replace(child);
        self.children.insert(i + 1, Some(sibling));
        self.set_key_count(self.key_count() + 1);
    }

    pub(crate) fn insert_nonfull(&mut self, value: T, order: usize) {
        let pos = self.key.iter().rev().position(|k| k.key() < value.key());
        match self.is_leaf() {
            true => {
                match pos {
                    Some(i) => self.key.insert(i + 1, value),
                    None => self.key.push(value),
                }
            }
            false => {
                if let Some(i) = pos {
                    let mut i = i + 1;
                    if self.children[i].as_ref().unwrap().is_full(order) {
                        self.split_child(i, order);
                        if value.key() > self.key[i].key() {
                            i += 1;
                        }
                    }
                    self.children[i].as_mut().unwrap().insert_nonfull(value, order);
                }
            }
        }
    }
}
