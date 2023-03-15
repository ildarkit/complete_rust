use std::fmt;
use std::ops::Not;
use std::cmp::Ordering;
use std::cell::{Ref, RefCell};
use std::rc::Rc;

pub type Tree<T> = Option<BareTree<T>>;
type RefCellNode<T> = Rc<RefCell<Node<T>>>;

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
pub struct Node<T: Default + Copy + Clone + fmt::Debug> {
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
        T: Default + Copy + Clone + fmt::Debug
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

fn repr<T: Default + Copy + Clone + fmt::Debug>(node: Tree<T>) -> Option<(T, Color)> {
    if let Some(n) = node {
        Some((n.key(), n.color()))
    } else { None }
}

impl<T> PartialEq for Node<T> 
    where 
        T: PartialEq + Default + Copy + Clone + fmt::Debug
{
    fn eq(&self, other: &Node<T>) -> bool {
        self.key == other.key
    }
}

impl<T> PartialOrd for Node<T> 
    where
        T: PartialOrd + Default + Copy + Clone + fmt::Debug
{
    fn partial_cmp(&self, other: &Node<T>) -> Option<Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct BareTree<T: Default + Copy + Clone + fmt::Debug> {
    node: RefCellNode<T>,
}

impl<T> BareTree<T>
    where 
        T: Default + Copy + Clone + fmt::Debug
{
    pub fn new(id: u32, key: T) -> Self {
        Self {
            node: Rc::new(RefCell::new(Node::new(
                id,
                key,
            )))
        }
    }

    pub fn key(&self) -> T {
        self.node.borrow().key
    }

    pub fn set_left_child(&mut self, node: Tree<T>) {
        self.node.borrow_mut().left = node.clone();
    }

    pub fn set_right_child(&mut self, node: Tree<T>) {
        self.node.borrow_mut().right = node.clone();
    }

    pub fn set_parent(&mut self, node: Tree<T>) {
        self.node.borrow_mut().parent = node.clone();
    }

    pub fn left_child(&self) -> Tree<T> {
        self.node.borrow().left.clone()
    }

    pub fn right_child(&self) -> Tree<T> {
        self.node.borrow().right.clone()
    } 

    pub fn unwrap_left_child(&self) -> BareTree<T> {
        self.left_child().as_ref().unwrap().clone()
    }

    pub fn unwrap_right_child(&self) -> BareTree<T> {
        self.right_child().as_ref().unwrap().clone()
    }

    pub fn parent(&self) -> Tree<T> {
        self.node.borrow().parent.clone()
    }

    pub fn unwrap_parent(&self) -> BareTree<T> {
        self.parent().as_ref().unwrap().clone()
    }

    pub fn id(&self) -> u32 {
        self.node.borrow().id
    }

    pub fn color(&self) -> Color {
        self.node.borrow().color.clone()
    }

    pub fn set_color(&mut self, color: Color) {
        self.node.borrow_mut().color = color;
    }

    pub fn borrow(&self) -> Ref<'_, Node<T>> {
        self.node.borrow()
    }
}
