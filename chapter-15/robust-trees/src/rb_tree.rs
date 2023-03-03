use std::ops::Not;
use std::cmp::Ordering;
use std::cell::RefCell;
use std::rc::Rc;

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
pub struct Node<T: Copy + Clone> {
    pub color: Color,
    pub key: T,
    parent: Tree<T>,
    left: Tree<T>,
    right: Tree<T>,
}

impl<T: Default + Copy + Clone> Node<T> {
    fn default_from(&self) -> Self {
        Self {
            color: self.color,
            key: self.key,
            ..Default::default()
        }
    }
}

impl<T: PartialEq + Copy + Clone> PartialEq for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool {
        self.key == other.key
    }
}

impl<T: PartialOrd + Copy + Clone> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Node<T>) -> Option<Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

#[derive(Default)]
pub struct RedBlackTree<T: Copy + Clone> {
    root: Tree<T>,
    pub length: u64,
}

impl<T: Default + PartialEq + PartialOrd + Copy + Clone> RedBlackTree<T> {
    pub fn new() -> Self {
        Self { ..Default::default() }
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

    pub fn insert(&mut self, value: T) {
        self.length += 1;
        let mut parent: BareTree<T> = Default::default();
        let new_node = Self::create_node(value);

        self.root = if let Some(root) = self.root.take() {
            parent = Self::find_parent(
                Some(root.clone()),
                new_node.clone()
            );
            Self::to_node(root)
        } else { None };
        
        if self.root.is_none() {
            self.root = Self::to_node(new_node.clone());
        } else {
            new_node.borrow_mut().parent = Self::to_node(parent.clone());
            if new_node.clone() < parent.clone() {
                parent.borrow_mut().left = Self::to_node(new_node.clone());
            } else {
                parent.borrow_mut().right = Self::to_node(new_node.clone());
            }
        }

        self.fixup(new_node);
    }

    pub fn find(&self, value: T) -> Option<Node<T>> {
        let value = Self::create_node(value);
        let mut current = self.root.clone();
        while let Some(node) = current {
            if node.clone() == value.clone() {
                let inner_node = node.replace(Default::default());
                let res = Some(inner_node.default_from());
                node.replace(inner_node);
                return res;
            }
            if value <= node.clone() {
                current = node.borrow().left.clone();
            } else {
                current = node.borrow().right.clone();
            }
        }
        None
    }

    fn fixup(&mut self, inserted: BareTree<T>) {
        let mut current = inserted.clone();
        while let Some(ref parent) = current.clone().borrow().parent {
            match parent.borrow().color {
                Color::Red => {
                    let child = Self::current_is_child(current.clone());
                    let rotations;
                    match child.clone() {
                        Child::Left => {
                            rotations = (Rotation::Left, Rotation::Right);
                        },
                        Child::Right => {
                            rotations = (Rotation::Right, Rotation::Left);
                        },
                    }
                    current = self.fix_subtree(parent, current, &!child, rotations);
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
        parent: &BareTree<T>,
        current: BareTree<T>,
        child: &Child,
        rotations: (Rotation, Rotation)
    ) -> BareTree<T> {
        let uncle = Self::find_uncle(current.clone(), child);
        let mut current = current.clone();
        match uncle.borrow().color {
            Color::Red => {
                parent.borrow_mut().color = Color::Black;
                uncle.borrow_mut().color = Color::Black;
                parent.borrow()
                    .parent.as_ref().unwrap().borrow_mut()
                    .color = Color::Red;
                current = parent.borrow()
                    .parent.as_ref().unwrap().clone();
            }
            _ => {
                let rotate_child = !child.to_owned();
                if Self::is_child(current.clone(), child) {
                    current = parent.clone();
                    self.rotate(current.clone(), rotations.0, &rotate_child);
                }
                current.borrow()
                    .parent.as_ref().unwrap().borrow_mut()
                    .color = Color::Black;
                let grandparent = current.borrow()
                    .parent.as_ref().unwrap().borrow()
                    .parent.as_ref().unwrap().clone();
                grandparent.borrow_mut().color = Color::Red;
                self.rotate(grandparent, rotations.1, &rotate_child);
            }
        }
        current.clone()
    } 

    fn find_uncle(node: BareTree<T>, child: &Child) -> BareTree<T> {
        let grand_parent = node.borrow()
            .parent.as_ref().unwrap().borrow()
            .parent.as_ref().unwrap().clone();
        match child {
            Child::Right => grand_parent.borrow().right.as_ref().unwrap().clone(),
            Child::Left => grand_parent.borrow().left.as_ref().unwrap().clone(),
        }
    }

    fn is_child(node: BareTree<T>, child: &Child) -> bool {
        let parent = node.borrow().parent.as_ref().unwrap().clone();
        match child {
            Child::Right => node == parent.borrow().right.as_ref().unwrap().clone(),
            Child::Left => node == parent.borrow().left.as_ref().unwrap().clone(),
        }
    }

    fn rotate(&mut self, node: BareTree<T>, rotation: Rotation, child: &Child) {
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

        if let Some(ref npc) = new_parent_child {
            npc.borrow_mut().parent = Self::to_node(node.clone());
        } 
        if let Some(ref np) = new_parent {
            np.borrow_mut().parent = node.borrow().parent.clone();
        }

        match node_parent {
            None => self.root = new_parent.clone(),
            Some(ref node_parent) => {
                match child {
                    Child::Left => {
                        node_parent.borrow_mut().left = new_parent.clone();
                        if let Some(ref np) = new_parent {
                            np.borrow_mut().left = Self::to_node(node.clone());
                        }
                    }
                    Child::Right => {
                        node_parent.borrow_mut().right = new_parent.clone();
                        if let Some(ref np) = new_parent {
                            np.borrow_mut().right = Self::to_node(node.clone());
                        }
                    }
                }
            }
        }
        node.borrow_mut().parent = new_parent.clone(); 
    } 
}
