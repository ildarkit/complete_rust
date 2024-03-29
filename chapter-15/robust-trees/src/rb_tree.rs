use std::fmt;
use std::cell::Ref;
use log::debug;

use crate::node::{
    Tree, BareTree, Child,
    Rotation, Color, Node,
}; 

#[derive(Default)]
pub struct RedBlackTree<T: Default + Copy + Clone + fmt::Debug> {
    root: Tree<T>,
    length: usize,
    id_node: u32,
}

impl<T> RedBlackTree<T> 
    where 
        T: Default + PartialEq + PartialOrd + Copy + Clone + fmt::Debug
{
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn insert(&mut self, value: T) {
        self.length += 1;
        let mut parent: BareTree<T> = Default::default();
        let new_node = self.create_node(value);

        self.root = if let Some(root) = self.root.take() {
            parent = Self::find_parent(
                &Some(root.clone()),
                &new_node
            );
            Self::to_node(&root)
        } else { None };
        
        if self.root.is_none() {
            self.root = Self::to_node(&new_node);
        } else {
            new_node.set_parent(Self::to_node(&parent));
            if new_node.clone() < parent.clone() {
                parent.set_left_child(Self::to_node(&new_node));
            } else {
                parent.set_right_child(Self::to_node(&new_node));
            }
        }
        debug!("Inserted node:\n {:#?}", new_node.clone());
        self.insert_fixup(&mut Self::to_node(&new_node));
        debug!("Fixed node:\n {:#?}", new_node.clone());
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
        let value = Self::new_node(0, value);
        let mut current = self.root.clone();
        while let Some(node) = current.clone() {
            if node.clone() == value.clone() {
                return Self::to_node(&node);
            }
            if value.clone() < node {
                current = node.left_child();
            } else {
                current = node.right_child();
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
            self.go_walk_in_order(&n.left_child(), callback);
            callback(&n.borrow());
            self.go_walk_in_order(&n.right_child(), callback);
        }
    }

    fn create_node(&mut self, key: T) -> BareTree<T> {
        self.id_node +=1;
        Self::new_node(self.id_node, key) 
    }

    fn new_node(id: u32, key: T) -> BareTree<T> {
        BareTree::new(id, key)
    }

    fn to_node(node: &BareTree<T>) -> Tree<T> {
        Some(node.clone())
    }

    fn find_parent(current: &Tree<T>, new_node: &BareTree<T>) -> BareTree<T> {
        let mut parent = Default::default();
        let mut current = current.clone();
        while let Some(ref node) = current {
            parent = node.clone();
            if new_node < node {
                current = node.left_child();
            } else {
                current = node.right_child();
            }
        }
        parent
    }

    fn insert_fixup(&mut self, inserted: &Tree<T>) {
        let mut current = inserted.clone();
        while let Some(node) = current {
            if node.color() == Color::Black {
                break;
            }
            let color = if let Some(parent) = node.parent() {
                parent.color()
            } else {
                break;
            };
            match color {
                Color::Red => {
                    let child = Self::node_is_child(&node); 
                    current = self.insert_fixup_subtree(
                        &mut Self::to_node(&node),
                        &child.unwrap()
                    );
                },
                Color::Black => break,
            }
        }
        self.root.as_ref().unwrap().set_color(Color::Black);
    }

    fn node_is_child(node: &BareTree<T>) -> Option<Child> {
        let parent = node.parent();
        match parent {
            Some(ref p) => {
                let is_left = match p.left_child() {
                    Some(child) => child.id() == node.id(),
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

    fn insert_fixup_subtree(&mut self, current: &Tree<T>, child: &Child)
        -> Tree<T>
    {
        let uncle = Self::find_uncle(&current);
        let mut current = current.clone();

        let mut parent = current.as_ref().unwrap().unwrap_parent();
        let uncle_is_red = Self::uncle_is_red(&uncle);
        if uncle_is_red {
            parent.set_color(Color::Black);
            uncle.as_ref().unwrap().set_color(Color::Black);
            parent.unwrap_parent().set_color(Color::Red);
            current = parent.parent();
        } else {
            let rotation = Self::get_rotation(child);
            let parent_is_child = Self::node_is_child(&parent);
            if parent_is_child.is_some() && *child == parent_is_child.unwrap() {
                let mut rotate_node = parent.unwrap_parent();
                rotate_node.set_color(Color::Red);
                parent.set_color(Color::Black);
                self.rotate(&mut rotate_node, &rotation);
            } else {
                self.rotate(&mut parent, &rotation);
            };
            current = Self::to_node(&parent);
        } 
        current
    }

    fn find_uncle(node: &Tree<T>) -> Tree<T> {
        let parent = &node.as_ref().unwrap().parent();
        let grand_parent = match parent {
            Some(p) => {
                match p.parent() {
                    Some(pp) => Self::to_node(&pp),
                    None => None,
                }
            },
            None => None,
        };
        if grand_parent.is_none() {
            return None;
        }     
        match Self::node_is_child(parent.as_ref().unwrap()) {
            Some(Child::Right) => grand_parent.as_ref().unwrap().left_child(),
            Some(Child::Left) => grand_parent.as_ref().unwrap().right_child(),
            _ => None,
        }
    }

    fn uncle_is_red(uncle: &Tree<T>) -> bool {
        match uncle {
            Some(n) => n.color() == Color::Red,
            None => false, 
        }
    } 

    fn get_rotation(child: &Child) -> Rotation {
        match child {
            Child::Left => Rotation::Right,
            Child::Right => Rotation::Left,
        }
    }

    fn rotate(&mut self, node: &mut BareTree<T>, rotation: &Rotation) {
        debug!("\nrotation = {:?}", rotation);
        debug!("\nnode = {:#?}", node);
        let (node_parent, new_parent, new_parent_child) = match rotation {
            Rotation::Left => {
                let new_parent = node.unwrap_right_child();
                let new_parent_child = new_parent.left_child();
                node.set_right_child(new_parent_child.clone());
                (node.parent(),
                    new_parent,
                    new_parent_child)
            }
            Rotation::Right => {
                let new_parent = node.unwrap_left_child();
                let new_parent_child = new_parent.right_child();
                node.set_left_child(new_parent_child.clone());
                (node.parent(),
                    new_parent,
                    new_parent_child)
            }
        };
        debug!("\nnode_parent(node.p) = {:#?}", node_parent);
        debug!("\nnew_parent(node_child) = {:#?}", new_parent);
        debug!("\nnode = {:#?}", node);
        debug!("\nnew_parent_child(child_child) = {:#?}", new_parent_child);

        if let Some(npc) = new_parent_child.clone() {
            npc.set_parent(Self::to_node(&node));
        } 
        new_parent.set_parent(node_parent.clone());
        debug!("\nnode_child.p = node.p: {:#?}", new_parent);

        match node_parent {
            None => self.root = Self::to_node(&new_parent),
            Some(node_parent) => {
                match Self::node_is_child(&node) {
                    Some(Child::Left) => {
                        node_parent.set_left_child(Self::to_node(&new_parent)); 
                        debug!("\nnew left child in node.p = {:#?}", node_parent);
                    }
                    Some(Child::Right) => {
                        node_parent.set_right_child(Self::to_node(&new_parent));
                        debug!("\nnew right child in node.p = {:#?}", node_parent);
                    }
                    None => { unreachable!() }
                }
            }
        }
        match rotation {
            Rotation::Right => {
                new_parent.set_right_child(Self::to_node(&node));
            },
            Rotation::Left => {
                new_parent.set_left_child(Self::to_node(&node));
            },
        }
        node.set_parent(Self::to_node(&new_parent)); 
        debug!("\nnew_parent(node_child) = {:#?}", new_parent);
        debug!("\nnode = {:#?}", node);
        debug!("\nnew_parent_child(child_child) = {:#?}", new_parent_child);
        debug!("\nend rotate fn");
    }

    fn replace(&mut self, removable: &mut BareTree<T>, replacement: &mut Tree<T>) {
        let parent = removable.parent();
        match parent {
            None => self.root = replacement.clone(),
            Some(p) => {
                let is_left_child = Self::node_is(
                    &removable,
                    &p.left_child()
                );
                match is_left_child {
                    true => p.set_left_child(replacement.clone()),
                    false => p.set_right_child(replacement.clone()),
                }
            }
        }
        if let Some(ref mut node) = replacement {
            node.set_parent(removable.parent());
        }
        removable.clear();
    }

    fn node_is(node: &BareTree<T>, other: &Tree<T>) -> bool {
        match other {
            Some(ref n) => n.id() == node.id(),
            None => false,
        }
    }

    pub fn delete(&mut self, value: T) -> bool {
        let mut deleted = match self.find_node(value) {
            Some(node) => node.clone(),
            None => return false,
        };
        debug!("\nnode for delete = {:#?}", deleted);
        let mut node_color = deleted.color();

        let replaced = match Self::get_if_one_child(&deleted) {
            Some(child) => {
                let mut child = Self::to_node(&child);
                self.replace(&mut deleted, &mut child);
                child
            }
            // node has two or no childrens
            None => {
                let replace = match Self::tree_minimum(&deleted.right_child()) {
                    Some(mut replace) => {
                        let mut replace_child = replace.right_child();
                        debug!("\nreplace = {:#?}", replace);
                        node_color = replace.color();
                        deleted.set_key(replace.key());
                        debug!("\nreplace node child = {:#?}", replace_child);
                        self.replace(&mut replace, &mut replace_child);
                        replace_child
                    }
                    None => {
                        self.replace(&mut deleted, &mut None);
                        None
                    }
                };
                replace
            }
        };
        if node_color == Color::Black {
            self.delete_fixup(&replaced);
        }
        self.length -=1;
        true
    }

    fn get_if_one_child(node: &BareTree<T>) -> Tree<T> {
        if node.left_child().is_none() {
            node.right_child()
        } else if node.right_child().is_none() {
            node.left_child()
        } else { None }
    }

    fn tree_minimum(node: &Tree<T>) -> Tree<T> {
        let mut node = node.clone();
        let mut result = None;
        while let Some(n) = node.clone() {
            result = node.clone();
            node = n.left_child();
        }
        result
    }

    fn delete_fixup(&mut self, node: &Tree<T>) {
        let mut node = node.clone();
        while let Some(n) = node.clone() {
            if n.id() != self.root.as_ref().unwrap().id()
                && n.color() == Color::Black
            {
                let child = Self::node_is_child(&n);
                let (sibling, rotation) =
                    match child.clone() {
                        Some(Child::Left) => {
                            (n.unwrap_parent().right_child(),
                                Rotation::Left)
                        },
                        Some(Child::Right) => {
                            (n.unwrap_parent().left_child(),
                                Rotation::Right)
                        },
                        None => { unreachable!() },
                    };
                node = self.delete_fixup_subtree(
                    &n, &sibling,
                    &rotation,
                    &child.unwrap(),
                );
            } else { 
                if n.color() == Color::Red {
                    n.set_color(Color::Black);
                }
                break;
            }; 
        }
        if self.root.is_some() {
            self.root.as_ref().unwrap().set_color(Color::Black);
        }
    }

    fn delete_fixup_subtree(&mut self, node: &BareTree<T>, sibling: &Tree<T>,
        rotation: &Rotation, node_is_child: &Child) -> Tree<T>
    {
        let node = node.clone();
        let mut sibling = sibling.clone();
        let mut parent = node.unwrap_parent();
        if sibling.is_some() && sibling.as_ref().unwrap().color() == Color::Red {
            sibling.as_ref().unwrap().set_color(Color::Black);
            parent.set_color(Color::Red);
            self.rotate(&mut parent, &rotation.clone());
            sibling = match node_is_child {
                Child::Left => parent.right_child(),
                Child::Right => parent.left_child(),
            };
        }

        let nephews = Self::childrens(&sibling, &node_is_child);
        let red_nephews = Self::any_colors(&nephews, &Color::Red);

        match red_nephews {
            false => {
                if sibling.is_some() {
                    sibling.unwrap().set_color(Color::Red);
                }
                node.parent()
            }
            true => {
                let close_nephew = nephews[0].clone();
                let distant_nephew = nephews[1].clone();
                let distant_black = Self::any_colors(
                    &vec![distant_nephew.clone()],
                    &Color::Black
                );
                
                if distant_black {
                    close_nephew.as_ref().unwrap().set_color(Color::Black);
                    sibling.as_ref().unwrap().set_color(Color::Red);
                    self.rotate(&mut sibling.unwrap(), &!rotation.clone());
                    match node_is_child {
                        Child::Left => sibling = node.unwrap_parent().right_child(),
                        Child::Right => sibling = node.unwrap_parent().left_child(),
                    }
                } else if let Some(distant_nephew) = distant_nephew {
                    distant_nephew.set_color(Color::Black);
                }
                sibling.unwrap().set_color(
                    node.unwrap_parent().color());
                node.unwrap_parent().set_color(Color::Black); 
                self.rotate(&mut node.unwrap_parent(), &rotation.clone());
                self.root.clone()
            }
        }
    }

    fn childrens(node: &Tree<T>, first_child: &Child) -> Vec<Tree<T>> {
        let (left, right) = match node {
            Some(n) => (n.left_child(), n.right_child()),
            None => (None, None),
        };
        match first_child {
            Child::Left => vec![left, right],
            Child::Right => vec![right, left],
        }
    }

    fn any_colors(nodes: &Vec<Tree<T>>, color: &Color) -> bool {
        nodes.iter()
            .any(|node| {
                if let Some(n) = node {
                    n.color() == *color
                } else { false }
            })
    }
}
