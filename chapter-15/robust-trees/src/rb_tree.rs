use std::fmt;
use std::ops::Not;
use std::cmp::Ordering;
use std::cell::{Ref, RefCell};
use std::rc::Rc;
use log::debug;

pub type BareTree<T> = Rc<RefCell<Node<T>>>;
pub type Tree<T> = Option<BareTree<T>>;

#[derive(PartialEq, Copy, Clone)]
enum Child {
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

enum Rotation {
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
    pub color: Color,
    pub key: T,
    parent: Tree<T>,
    left: Tree<T>,
    right: Tree<T>,
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

#[derive(Default)]
pub struct RedBlackTree<T: Copy + Clone + fmt::Debug> {
    root: Tree<T>,
    pub length: u64,
}

impl<T> RedBlackTree<T> 
    where 
        T: Default + PartialEq + PartialOrd + Copy + Clone + fmt::Debug
{
    pub fn new() -> Self {
        Self { ..Default::default() }
    } 

    pub fn insert(&mut self, value: T) {
        self.length += 1;
        let mut parent: BareTree<T> = Default::default();
        let new_node = Self::to_node(Self::create_node(value));

        self.root = if let Some(root) = self.root.take() {
            parent = Self::find_parent(
                Some(root.clone()),
                new_node.as_ref().unwrap().clone()
            );
            Self::to_node(root)
        } else { None };
        
        if self.root.is_none() {
            self.root = new_node.clone();
        } else {
            new_node.as_ref().unwrap().borrow_mut()
                .parent = Self::to_node(parent.clone());
            if new_node.as_ref().unwrap().clone() < parent.clone() {
                parent.borrow_mut().left = new_node.clone();
            } else {
                parent.borrow_mut().right = new_node.clone();
            }
        }
        debug!("Inserted node:\n {:#?}", new_node.as_ref().unwrap().borrow());
        self.fixup(new_node.clone());
    }

    pub fn find(&self, value: T,
        mut callback: impl FnMut(Option<&Ref<Node<T>>>) -> ())
    {
        let value = Self::create_node(value);
        let mut current = self.root.clone();
        while let Some(ref node) = current.clone() {
            let node = node.clone();
            if node == value.clone() {
                return callback(Some(&node.borrow()));
            }
            if value.clone() <= node {
                current = node.borrow().left.clone();
            } else {
                current = node.borrow().right.clone();
            }
        }
        callback(None);
    }

    pub fn walk_in_order(&self,
        mut callback: impl FnMut(&Ref<'_, Node<T>>) -> ())
    {
        self.go_walk_in_order(&self.root, &mut callback);
    }

    fn go_walk_in_order(&self, node: &Tree<T>,
        callback: &mut impl FnMut(&Ref<'_, Node<T>>) -> ())
    {
        if let Some(n) = node {
            let n = n.borrow();
            self.go_walk_in_order(&n.left, callback);
            callback(&n);
            self.go_walk_in_order(&n.right, callback);
        }
    }

    fn create_node(key: T) -> BareTree<T> {
        Rc::new(RefCell::new(Node {
            key,
            ..Default::default() 
        }))
    }

    fn to_node(node: BareTree<T>) -> Tree<T> {
        Some(node)
    }

    fn find_parent(current: Tree<T>, new_node: BareTree<T>) -> BareTree<T> {
        let mut parent = Default::default();
        let mut current = current.clone();
        while let Some(ref node) = current.clone() {
            parent = node.clone();
            if new_node < node.clone() {
                current = node.borrow().left.clone();
            } else {
                current = node.borrow().right.clone();
            }
        }
        parent.clone()
    }

    fn fixup(&mut self, inserted: Tree<T>) {
        let mut current = inserted.clone();
        let mut rotations;
        while let Some(node) = current.clone() {
            let color = if let Some(parent) = node.borrow().parent.clone() {
                parent.borrow().color
            } else {
                break;
            };
            match color {
                Color::Red => {
                    let child = Self::current_is_child(node.clone());
                    match child.clone() {
                        Child::Left => {
                            rotations = (Rotation::Left, Rotation::Right);
                        },
                        Child::Right => {
                            rotations = (Rotation::Right, Rotation::Left);
                        },
                    }
                    current = self.fix_subtree(Self::to_node(node.clone()),
                        &!child, rotations);
                },
                Color::Black => break,
            }
        }
        self.root.as_ref().unwrap().borrow_mut().color = Color::Black;
    }

    fn current_is_child(node: BareTree<T>) -> Child {
        let left  = match node.borrow().parent {
            Some(ref parent) => parent.borrow().left.clone(),
            None => None,
        };
        let is_left = match left {
            Some(ref left) => left.clone() == node.clone(),
            None => false,
        };
        match is_left {
            true => Child::Left,
            false => Child::Right,
        }
    }

    fn fix_subtree(
        &mut self,
        current: Tree<T>,
        child: &Child,
        rotations: (Rotation, Rotation)
    ) -> Tree<T> {
        let uncle = Self::find_uncle(current.clone(), child);
        let mut current = current.clone();

        if let Some(uncle) = uncle.clone() {
            let parent = current.as_ref().unwrap().borrow()
                .parent.as_ref().unwrap().clone();
            let uncle_is_red = Self::uncle_is_red(uncle.clone());
            if uncle_is_red {
                parent.borrow_mut().color = Color::Black;
                uncle.borrow_mut().color = Color::Black;
                parent.borrow()
                    .parent.as_ref().unwrap().borrow_mut()
                    .color = Color::Red;
                current = parent.borrow()
                    .parent.clone();
            } else {
                let rotate_child = !child.to_owned();
                if Self::is_child(current.clone(), child) {
                    current = Self::to_node(parent.clone());
                    self.rotate(current.clone(), rotations.0, &rotate_child);
                }
                current.as_ref().unwrap().borrow()
                    .parent.as_ref().unwrap().borrow_mut()
                    .color = Color::Black;
                let grandparent = current.as_ref().unwrap().borrow()
                    .parent.as_ref().unwrap().borrow()
                    .parent.clone();
                grandparent.as_ref().unwrap().borrow_mut().color = Color::Red;
                self.rotate(grandparent, rotations.1, &rotate_child);
            }
        } else {
            current = self.root.clone()
        } 
        current
    } 

    fn find_uncle(node: Tree<T>, child: &Child) -> Tree<T> {
        let grand_parent = match node.as_ref().unwrap().borrow().parent.clone() {
            Some(p) => {
                match p.borrow().parent.clone() {
                    Some(pp) => Self::to_node(pp.clone()),
                    None => None,
                }
            },
            None => None,
        };
        if grand_parent.is_none() {
            return None;
        }
        match child {
            Child::Right => grand_parent.unwrap().borrow().right.clone(),
            Child::Left => grand_parent.unwrap().borrow().left.clone(),
        }
    }

    fn uncle_is_red(uncle: BareTree<T>) -> bool {
        match uncle.borrow().color {
            Color::Red => true,
            _ => false, 
        }
    }

    fn is_child(node: Tree<T>, child: &Child) -> bool {
        let parent = node.as_ref().unwrap().borrow()
            .parent.as_ref().unwrap().clone();
        let child_node = match child {
            Child::Right => parent.borrow().right.clone(),
            Child::Left => parent.borrow().left.clone(),
        };
        match child_node {
            Some(n) => n == node.as_ref().unwrap().clone(),
            None => false,
        }
    }

    fn rotate(&mut self, node: Tree<T>, rotation: Rotation, child: &Child) {
        let node = node.unwrap();
        let (node_parent, new_parent, new_parent_child) = match rotation {
            Rotation::Left => {
                let new_parent = node.borrow().right.clone();
                let new_parent_child = match new_parent.clone() {
                    Some(ref p) => p.borrow().left.clone(),
                    None => None,
                };
                node.borrow_mut().right = new_parent_child.clone();
                (node.borrow().parent.clone(),
                    new_parent,
                    new_parent_child)
            }
            Rotation::Right => {
                let new_parent = node.borrow().left.clone();
                let new_parent_child = match new_parent.clone() {
                    Some(ref p) => p.borrow().right.clone(),
                    None => None,
                };
                node.borrow_mut().left = new_parent_child.clone();
                (node.borrow().parent.clone(),
                    new_parent,
                    new_parent_child)
            }
        };

        if let Some(npc) = new_parent_child.clone() {
            npc.borrow_mut().parent = Self::to_node(node.clone());
        } 
        if let Some(np) = new_parent.clone() {
            np.borrow_mut().parent = node.borrow().parent.clone();
        }

        match node_parent.clone() {
            None => self.root = new_parent.clone(),
            Some(node_parent) => {
                match child {
                    Child::Left => {
                        node_parent.borrow_mut().left = new_parent.clone();
                        if let Some(np) = new_parent.clone() {
                            np.borrow_mut().left = Self::to_node(node.clone());
                        }
                    }
                    Child::Right => {
                        node_parent.borrow_mut().right = new_parent.clone();
                        if let Some(np) = new_parent.clone() {
                            np.borrow_mut().right = Self::to_node(node.clone());
                        }
                    }
                }
            }
        }
        node.borrow_mut().parent = new_parent.clone(); 
    } 
}
