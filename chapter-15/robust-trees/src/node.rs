use std::fmt;
use std::ops::Not;
use std::cmp::Ordering;
use std::cell::RefCell;
use std::rc::Rc;

pub type BareTree<T> = Rc<RefCell<Node<T>>>;
pub type Tree<T> = Option<BareTree<T>>;

#[derive(PartialEq, Copy, Clone)]
pub enum Child {
    Left,
    Right,
}

impl Not for Child {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Child::Left => Child::Right,
            Child::Right => Child::Left,
        }
    }
}

pub enum Rotation {
    Left,
    Right,
}

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub enum Color {
    #[default]
    Red,
    Black,
}

#[derive(Default)]
pub struct Node<T: Copy + Clone + fmt::Debug> {
    pub id: u32,
    pub color: Color,
    pub key: T,
    pub parent: Tree<T>,
    pub left: Tree<T>,
    pub right: Tree<T>,
}

impl<T> Node<T> 
    where
        T: Default + Copy + Clone + fmt::Debug
{
    pub fn new(id: u32, key: T) -> Self {
        Self {
            id,
            key,
            ..Default::default()
        }
    }
} 

impl<T> fmt::Debug for Node<T> 
    where
        T: Copy + Clone + fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let parent = repr(self.parent.clone());
        let left = repr(self.left.clone());
        let right = repr(self.right.clone());
        f.debug_struct("Node")
            .field("key", &self.key)
            .field("color", &self.color)
            .field("parent", &parent)
            .field("left", &left)
            .field("right", &right)
            .finish()
    } 
}

fn repr<T: Copy + Clone + fmt::Debug>(node: Tree<T>) -> Option<(T, Color)> {
    if let Some(n) = node {
        Some((n.borrow().key, n.borrow().color))
    } else { None }
}

impl<T> PartialEq for Node<T> 
    where 
        T: PartialEq + Copy + Clone + fmt::Debug
{
    fn eq(&self, other: &Node<T>) -> bool {
        self.key == other.key
    }
}

impl<T> PartialOrd for Node<T> 
    where
        T: PartialOrd + Copy + Clone + fmt::Debug
{
    fn partial_cmp(&self, other: &Node<T>) -> Option<Ordering> {
        self.key.partial_cmp(&other.key)
    }
}
