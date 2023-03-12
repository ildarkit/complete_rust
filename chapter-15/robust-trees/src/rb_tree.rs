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
        self.insert_fixup(new_node.clone());
        debug!("Fixed node:\n {:#?}", new_node.as_ref().unwrap().borrow());
    }

    pub fn find(&self, value: T,
        mut callback: impl FnMut(Option<Ref<Node<T>>>) -> ())
    {
        let node = self.find_node(value);
        let node = match node {
            Some(ref n) => Some(n.borrow()),
            None => None,
        };
        callback(node);
    }

    fn find_node(&self, value: T) -> Tree<T> {
        let value = Self::create_node(value);
        let mut current = self.root.clone();
        while let Some(node) = current.clone() {
            if node.clone() == value.clone() {
                return Self::to_node(node.clone());
            }
            if value.clone() < node {
                current = node.borrow().left.clone();
            } else {
                current = node.borrow().right.clone();
            }
        }
        current
    }

    pub fn walk_in_order(&self,
        mut callback: impl FnMut(&Ref<'_, Node<T>>) -> ())
    {
        self.go_walk_in_order(&self.root, &mut callback);
    }

    fn go_walk_in_order(&self, node: &Tree<T>,
        callback: &mut impl FnMut(&Ref<'_, Node<T>>) -> ())
    {
        if let Some(ref n) = node.clone() {
            let n = n.clone();
            self.go_walk_in_order(&n.borrow().left.clone(), callback);
            callback(&n.borrow());
            self.go_walk_in_order(&n.borrow().right.clone(), callback);
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
            if new_node.clone() <= node.clone() {
                current = node.borrow().left.clone();
            } else {
                current = node.borrow().right.clone();
            }
        }
        parent.clone()
    }

    fn insert_fixup(&mut self, inserted: Tree<T>) {
        let mut current = inserted.clone();
        while let Some(node) = current.clone() {
            let color = if let Some(parent) = node.borrow().parent.clone() {
                parent.borrow().color
            } else {
                break;
            };
            match color {
                Color::Red => {
                    let child = Self::node_is_child(node.clone()); 
                    current = self
                        .insert_fixup_subtree(Self::to_node(node.clone()), &child.unwrap());
                },
                Color::Black => break,
            }
        }
        self.root.as_ref().unwrap().borrow_mut().color = Color::Black;
    }

    fn node_is_child(node: BareTree<T>) -> Option<Child> {
        let parent = node.borrow().parent.clone();
        match parent {
            Some(ref p) => { 
                let is_left = match p.borrow().left.clone() {
                    Some(ref child) => child.clone() == node.clone(),
                    None => false,
                };
                match is_left {
                    true => Some(Child::Left),
                    false => Some(Child::Right),
                }
            },
            None => None,
        }
    }

    fn insert_fixup_subtree(&mut self, current: Tree<T>, child: &Child)
        -> Tree<T>
    {
        let uncle = Self::find_uncle(current.clone());
        let mut current = current.clone();

        let parent = current.as_ref().unwrap().borrow()
            .parent.as_ref().unwrap().clone();
        let uncle_is_red = Self::uncle_is_red(uncle.clone());
        if uncle_is_red {
            parent.borrow_mut().color = Color::Black;
            uncle.as_ref().unwrap().borrow_mut().color = Color::Black;
            parent.borrow()
                .parent.as_ref().unwrap().borrow_mut()
                .color = Color::Red;
            current = parent.borrow()
                .parent.clone();
        } else {
            let rotation = Self::get_rotation(child);
            let parent_is_child = Self::node_is_child(parent.clone());
            if parent_is_child.is_some() && *child == parent_is_child.unwrap() {
                let rotate_node = parent.borrow().parent.as_ref().unwrap().clone();
                rotate_node.borrow_mut().color = Color::Red;
                parent.borrow_mut().color = Color::Black;
                self.rotate(rotate_node, &rotation);
            } else {
                self.rotate(parent.clone(), &rotation);
            };
            current = Self::to_node(parent.clone());
        } 
        current
    } 

    fn find_uncle(node: Tree<T>) -> Tree<T> {
        let parent = node.as_ref().unwrap().borrow().parent.clone();
        let grand_parent = match parent.clone() {
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
        match Self::node_is_child(parent.as_ref().unwrap().clone()) {
            Some(Child::Right) => grand_parent.as_ref().unwrap().borrow().left.clone(),
            Some(Child::Left) => grand_parent.as_ref().unwrap().borrow().right.clone(),
            _ => None,
        }
    }

    fn uncle_is_red(uncle: Tree<T>) -> bool {
        match uncle {
            Some(n) => n.borrow().color == Color::Red,
            None => false, 
        }
    } 

    fn get_rotation(child: &Child) -> Rotation {
        match child {
            Child::Left => Rotation::Right,
            Child::Right => Rotation::Left,
        }
    }

    fn rotate(&mut self, node: BareTree<T>, rotation: &Rotation) {
        debug!("\nnode = {:#?}", node);
        let (node_parent, new_parent, new_parent_child) = match rotation {
            Rotation::Left => {
                let new_parent = node.borrow().right.as_ref().unwrap().clone();
                let new_parent_child = new_parent.borrow().left.clone();
                node.borrow_mut().right = new_parent_child.clone();
                (node.borrow().parent.clone(),
                    new_parent,
                    new_parent_child)
            }
            Rotation::Right => {
                let new_parent = node.borrow().left.as_ref().unwrap().clone();
                let new_parent_child = new_parent.borrow().right.clone();
                node.borrow_mut().left = new_parent_child.clone();
                (node.borrow().parent.clone(),
                    new_parent,
                    new_parent_child)
            }
        };
        debug!("\nnode_parent(node.p) = {:#?}", node_parent);
        debug!("\nnew_parent(node_child) = {:#?}", new_parent);
        debug!("\nnode = {:#?}", node);
        debug!("\nnew_parent_child(child_child) = {:#?}", new_parent_child);

        if let Some(npc) = new_parent_child.clone() {
            npc.borrow_mut().parent = Self::to_node(node.clone());
        } 
        new_parent.borrow_mut().parent = node_parent.clone();
        debug!("\nnode_child.p = node.p: {:#?}", new_parent);

        match node_parent.clone() {
            None => self.root = Self::to_node(new_parent.clone()),
            Some(node_parent) => {
                match Self::node_is_child(node.clone()) {
                    Some(Child::Left) => {
                        node_parent.borrow_mut().left = Self::to_node(new_parent.clone()); 
                        debug!("\nnew left child in node.p = {:#?}", node_parent);
                    }
                    Some(Child::Right) => {
                        node_parent.borrow_mut().right = Self::to_node(new_parent.clone());
                        debug!("\nnew right child in node.p = {:#?}", node_parent);
                    }
                    None => { unreachable!() }
                }
            }
        }
        match rotation {
            Rotation::Right => {
                new_parent.borrow_mut().right = Self::to_node(node.clone());
            },
            Rotation::Left => {
                new_parent.borrow_mut().left = Self::to_node(node.clone());
            },
        }
        node.borrow_mut().parent = Self::to_node(new_parent.clone()); 
        debug!("\nnew_parent(node_child) = {:#?}", new_parent);
        debug!("\nnode = {:#?}", node);
        debug!("\nnew_parent_child(child_child) = {:#?}", new_parent_child);
        debug!("\nend rotate fn");
    }

    fn transplant(&mut self, delete_node: BareTree<T>, replacement_node: BareTree<T>) {
        let parent = delete_node.borrow().parent.clone();
        match parent {
            None => self.root = Self::to_node(replacement_node.clone()),
            Some(p) => {
               if Self::is_replaceable(p.borrow().left.clone(), delete_node.clone()) {
                    p.borrow_mut().left = Self::to_node(replacement_node.clone());
                } else
                    if Self::is_replaceable(p.borrow().right.clone(), delete_node.clone())
                {
                    p.borrow_mut().right = Self::to_node(replacement_node.clone());
                }
            }
        }
        replacement_node.borrow_mut().parent = delete_node.borrow().parent.clone();
    }

    fn is_replaceable(
        replaceable: Tree<T>,
        delete_node: BareTree<T>) -> bool
    {
        match replaceable.clone() {
            Some(node) if node.clone() == delete_node.clone() => true,
            Some(_) | None => false,
        }
    } 

    pub fn delete(&mut self, value: T) -> bool {
        let child: BareTree<T>;
        let node = match self.find_node(value) {
            Some(node) => node.clone(),
            None => return false,
        };
        let mut node_color = node.borrow().color;

        match Self::is_one_child(node.clone()) {
            Some(n) => {
                child = n.clone(); 
                self.transplant(node.clone(), child.clone());
            }
            None => {
                let min_node = Self::tree_minimum(node.borrow().right.clone());
                node_color = min_node.borrow().color;
                child = min_node.borrow().right.as_ref().unwrap().clone();
                if min_node.borrow().parent.as_ref().unwrap().clone() == node.clone() {
                    child.borrow_mut().parent = Self::to_node(min_node.clone());
                } else {
                    self.transplant(
                        min_node.clone(),
                        min_node.borrow()
                            .right.as_ref().unwrap().clone()
                    );
                    min_node.borrow_mut().right = node.borrow().right.clone();
                    min_node.borrow()
                        .right.as_ref().unwrap().borrow_mut()
                        .parent = Self::to_node(min_node.clone());
                }
                self.transplant(node.clone(), min_node.clone());
                min_node.borrow_mut().left = node.borrow().left.clone();
                min_node.borrow()
                    .left.as_ref().unwrap().borrow_mut()
                    .parent = Self::to_node(min_node.clone());
                min_node.borrow_mut().color = node.borrow().color;
            }
        }
        if node_color == Color::Black {
            self.delete_fixup(Self::to_node(child.clone()));
        }
        true
    }

    fn is_one_child(node: BareTree<T>) -> Tree<T> {
        if node.borrow().left.is_none() {
            node.borrow().right.clone()
        } else if node.borrow().right.is_none() {
            node.borrow().left.clone()
        } else { None }
    }

    fn tree_minimum(node: Tree<T>) -> BareTree<T> {
        let mut node = node.clone();
        let mut result: BareTree<T> = Default::default();
        while let Some(n) = node {
            result = n.clone();
            node = n.borrow().left.clone();
        }
        result
    }

    fn is_color(node: BareTree<T>, color: Color) -> bool {
        node.borrow().color == color
    }

    fn delete_fixup(&mut self, node: Tree<T>) {
        let mut node = node.clone();
        while let Some(inner_node) = node.clone() {
            if inner_node.clone() != self.root.as_ref().unwrap().clone()
                && Self::is_color(inner_node.clone(), Color::Black)
            {
                let child = Self::node_is_child(inner_node.clone());
                let parent_child = match child {
                    Some(Child::Left) => {
                        inner_node.borrow()
                            .parent.as_ref().unwrap().borrow()
                            .right.as_ref().unwrap().clone()
                    },
                    Some(Child::Right) => {
                        inner_node.borrow()
                            .parent.as_ref().unwrap().borrow()
                            .left.as_ref().unwrap().clone()
                    },
                    None => { break },
                };
                node = self.delete_fixup_subtree(inner_node.clone(),
                    parent_child.clone());
            } else { break }
        }
        self.root.as_ref().unwrap().borrow_mut().color = Color::Black;
    }

    fn delete_fixup_subtree(&mut self,
        node: BareTree<T>,
        parent_child: BareTree<T>)
        -> Tree<T>
    {
        let mut parent_child = parent_child.clone();
        let parent_node = node.borrow().parent.as_ref().unwrap().clone();

        if Self::is_color(parent_child.clone(), Color::Red) {
            parent_child.borrow_mut().color = Color::Black;
            parent_node.borrow_mut().color = Color::Red;
            self.rotate(parent_node.clone(), &Rotation::Left);
            parent_child = node.borrow().parent.as_ref().unwrap().clone();
        }
        if Self::is_color(
            parent_child.borrow().left.as_ref().unwrap().clone(),
            Color::Black) &&
            Self::is_color(
                parent_child.borrow().right.as_ref().unwrap().clone(),
                Color::Black)
        {
            parent_child.borrow_mut().color = Color::Red;
            node.borrow().parent.clone()
        } else {
            if Self::is_color(
                parent_child.borrow()
                    .right.as_ref().unwrap().clone(),
                Color::Black)
            {
                parent_child.borrow()
                    .left.as_ref().unwrap().borrow_mut().color = Color::Black;
                parent_child.borrow_mut().color = Color::Red;
                self.rotate(parent_child.clone(), &Rotation::Right);
                parent_child = node.borrow()
                    .parent.as_ref().unwrap().borrow()
                    .right.as_ref().unwrap().clone();
            }
            parent_child.borrow_mut()
                .color = node.borrow().parent.as_ref().unwrap().borrow().color;
            node.borrow().parent.as_ref().unwrap().borrow_mut().color = Color::Black;
            parent_child.borrow().right.as_ref().unwrap().borrow_mut().color = Color::Black;
            self.rotate(node.borrow().parent.as_ref().unwrap().clone(), &Rotation::Left);
            self.root.clone()
        }
    }
}
