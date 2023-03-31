use std::fmt;
use std::marker::PhantomData;
use std::hash::Hash;
use std::ops::{Not, AddAssign};
use log::debug;

use crate::node::{Color, Operations};
use crate::repo::Repository;

type DefaultId = u32;

#[derive(Debug, Clone)]
struct NodeColor<U> {
    id: U,
    color: Color,
}

struct Relative<U = DefaultId> 
    where
        U: Eq + Hash + Default + fmt::Debug + Copy + PartialOrd
{
    parent: Option<U>,
    child: U,
    grandchild: Option<U>,
}

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

#[derive(Debug, Copy, Clone)]
enum Rotation {
    Left,
    Right,
}

impl Not for Rotation {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Rotation::Left => Rotation::Right,
            Rotation::Right => Rotation::Left,
        }
    }
}

#[derive(Default)]
pub struct RedBlackTree<'a, R, T, U = DefaultId> 
    where
        T: Eq + Hash + Default + Copy + Clone + fmt::Debug,
        U: Eq + Hash + Default + fmt::Debug + Copy + PartialOrd,
        R: Repository<T, U>,
{
    id_counter: U,
    key: PhantomData<T>,
    nodes: PhantomData<&'a R>,
    root: Option<U>,
    length: usize,
}

impl<R, T, U> RedBlackTree<'_, R, T, U> 
    where
        T: Eq + Hash + Default + PartialEq + PartialOrd + Copy + Clone + fmt::Debug,
        U: Eq + Hash + Default + fmt::Debug + Copy + PartialOrd + AddAssign<DefaultId>,
        for<'a> R: Repository<T, U> + Default + 'a,
        R::Output: Operations<T, U> + fmt::Debug + PartialOrd + Copy + Clone,
{
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn insert(&mut self, repo: &mut R, value: T) {
        let new_id = &self.get_id();
        repo.add(new_id, value);
        
        match self.root {
            None => self.root = Some(*new_id),
            _ => {
                let parent = self.find_parent(repo, &self.root, new_id);
                let mut new_node = repo.get_mut(new_id);
                new_node.as_mut().map(|n| {
                    n.set_parent(&parent);
                    debug!("Inserted node:\n {:#?}", n);
                });
                match parent {
                    Some(parent) => {
                        let new_key = new_node.unwrap().key();
                        let parent = repo.get_mut(&parent).unwrap();
                        if new_key < parent.key() {
                            parent.set_left_child(&Some(*new_id));
                        } else {
                            parent.set_right_child(&Some(*new_id));
                        }
                    }
                    _ => ()
                }
            }
        }
        self.insert_fixup(repo, new_id);
        self.inc_len();
        debug!("Fixed node:\n {:#?}", repo.get(new_id));
    }

    fn get_id(&mut self) -> U {
        self.id_counter += 1;
        self.id_counter
    }

    fn inc_len(&mut self) {
        self.length += 1;
    }

    fn dec_len(&mut self) {
        self.length -= 1;
    }

    pub fn find(&self, repo: &R, value: T,
        mut callback: impl FnMut(Option<R::Output>) -> ())
    {
        let node = self.find_node(repo, value);
        callback(node);
    }

    fn find_node(&self, repo: &R, value: T) -> Option<R::Output> {
        let mut current = self.root.clone();
        while let Some(ref node_id) = current {
            let node = repo.get(node_id)?;
            if node.key() == value {
                return Some(node.clone());
            }
            if value < node.key() {
                current = node.left_child();
            } else {
                current = node.right_child();
            }
        }
        None
    }

    pub fn walk_in_order(&self, repo: &R,
        mut callback: impl FnMut(&R::Output) -> ())
    {
        self.go_walk_in_order(repo, &self.root, &mut callback);
    }

    fn go_walk_in_order(&self, repo: &R, node: &Option<U>,
        callback: &mut impl FnMut(&R::Output) -> ())
    {
        if let Some(n) = node {
            let n = repo.get(n).unwrap();
            self.go_walk_in_order(repo, &n.left_child(), callback);
            callback(n);
            self.go_walk_in_order(repo, &n.right_child(), callback);
        }
    }

    fn find_parent(&self, repo: &R, current: &Option<U>, new_node: &U) -> Option<U> {
        let mut current = current.clone();
        let mut parent = None;
        let new_node = repo.get(new_node).unwrap();
        while let Some(node_id) = current {
            let node = repo.get(&node_id).unwrap();
            parent = Some(node_id);
            if new_node < node {
                current = node.left_child();
            } else {
                current = node.right_child();
            }
        }
        parent
    }

    fn insert_fixup(&mut self, repo: &mut R, inserted: &U) {
        let mut current = repo.get(inserted)
            .map(|node| NodeColor {id: *inserted, color: *node.color()});
        while let Some(NodeColor {id: node_id, color}) = current {
            if color == Color::Black {
                break;
            }
            let parent = repo.get_parent(&node_id);
            let parent_color = match parent {
                Some(p) => *p.color(),
                None => break,
            };
            current = match parent_color {
                Color::Red => {
                    let child = Self::node_is_child(repo, &node_id);
                    self.insert_fixup_subtree(repo, &node_id, &child.unwrap())
                },
                Color::Black => break,
            };
        }
        if let Some(root) = self.root {
            repo.get_mut(&root).unwrap().set_color(&Color::Black);
        }
    }

    fn node_is_child(repo: &R, node: &U) -> Option<Child> {
        let parent = repo.get_parent(node);
        match parent {
            Some(p) => {
                let is_left = match p.left_child() {
                    Some(child_id) => {
                        let child = repo.get(&child_id).unwrap();
                        *child.id() == *node
                    }
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

    fn insert_fixup_subtree(&mut self, repo: &mut R, current_id: &U, child: &Child)
        -> Option<NodeColor<U>>
    {
        let black = Color::Black;
        let parent_id = *repo.get_parent(current_id)?.id();
        let uncle = Self::find_uncle(repo, current_id);
        match Self::uncle_is_red(repo, &uncle) {
            true => {
                repo.get_mut(&parent_id).unwrap().set_color(&black);
                repo.get_mut(&uncle.unwrap()).unwrap().set_color(&black);
                let grandparent_id = *repo.get_parent(&parent_id).unwrap().id();
                repo.get_mut(&grandparent_id).unwrap().set_color(&!black);
                Some(NodeColor {
                    id: grandparent_id,
                    color: black,
                })
            }
            false => {
                let rotation = Self::get_rotation(child);
                let parent_is_child = Self::node_is_child(repo, &parent_id);
                if parent_is_child.is_some() && *child == parent_is_child.unwrap() {
                    let rotate_id = repo.mut_parent(&parent_id)
                        .map(|node| {
                            node.set_color(&!black);
                            *node.id()
                        });
                    repo.get_mut(&parent_id).unwrap().set_color(&black);
                    self.rotate(repo, &rotate_id.unwrap(), &rotation);
                } else {
                    self.rotate(repo, &parent_id, &rotation);
                };
                Some(NodeColor {
                    id: parent_id,
                    color: *repo.get(&parent_id).unwrap().color()
                })
            }
        }
    }

    fn find_uncle(repo: &R, node: &U) -> Option<U> {
        let parent = repo.get_parent(node);
        let grand_parent = parent
                .map(|p| repo.get_parent(p.id()))
                .flatten();
        match grand_parent {
            Some(gp) => {
                match Self::node_is_child(repo, parent.unwrap().id()) {
                    Some(Child::Right) => gp.left_child(),
                    Some(Child::Left) => gp.right_child(),
                    _ => None,
                }
            }
            None => None,
        }
    }

    fn uncle_is_red(repo: &R, uncle: &Option<U>) -> bool {
        match uncle {
            Some(n) => repo.get(n)
                .map_or(false, |n| *n.color() == Color::Red),
            None => false, 
        }
    } 

    fn get_rotation(child: &Child) -> Rotation {
        match child {
            Child::Left => Rotation::Right,
            Child::Right => Rotation::Left,
        }
    }

    fn get_relatives(&self, repo: &R, node: &U, rot: &Rotation)
        -> Relative<U>
    {
        let node = repo.get(node).unwrap();
        let (parent, child, grandchild) = match rot {
            Rotation::Left => {
                let child = &node.right_child().unwrap();
                (node.parent(),
                *child,
                repo.get_left(child).map(|node| *node.id()))
            }
            Rotation::Right => {
                let child = &node.left_child().unwrap();
                (node.parent(),
                *child,
                repo.get_right(child).map(|node| *node.id()))
            }
        };
        Relative {
            parent,
            child,
            grandchild,
        }
    }

    fn rotate(&mut self, repo: &mut R, current: &U, rotation: &Rotation) { 
        debug!("\nrotation = {:?}", rotation);
        let relative = self.get_relatives(repo, current, rotation);
        let grandchild = &relative.grandchild;
        repo.get_mut(current).map(|node| {
            debug!("\nnode = {:#?}", node);
            match rotation {
                Rotation::Left => {
                    node.set_right_child(grandchild);
                }
                Rotation::Right => {
                    node.set_left_child(grandchild);
                }
            };
            debug!("\nnode = {:#?}", node);
        });
        grandchild.as_ref().map(|node_id| {
            repo.get_mut(node_id)
            .map(|n| {
                n.set_parent(&Some(*current));
                debug!("\nnew_parent_child(child_child) = {:#?}", n);
            });
        });

        repo.get_mut(&relative.child).map(|child| {
            child.set_parent(&relative.parent);
            debug!("\nnode_child.p = node.p: {:#?}", child);
        });

        match relative.parent {
            None => self.root = Some(relative.child),
            Some(ref parent) => {
                let node_is = Self::node_is_child(repo, current);
                let parent = repo.get_mut(parent).unwrap();
                debug!("\nnode_parent(node.p) = {:#?}", parent);
                match node_is {
                    Some(Child::Left) => {
                        parent.set_left_child(&Some(relative.child)); 
                        debug!("\nnew left child in node.p = {:#?}", parent);
                    }
                    Some(Child::Right) => {
                        parent.set_right_child(&Some(relative.child));
                        debug!("\nnew right child in node.p = {:#?}", parent);
                    }
                    None => { unreachable!() }
                }
            }
        }

        repo.get_mut(&relative.child).map(|child| {
            match rotation {
                Rotation::Right => {
                    child.set_right_child(&Some(*current));
                },
                Rotation::Left => {
                    child.set_left_child(&Some(*current));
                },
            }
            debug!("\nnew_parent(node_child) = {:#?}", child);
        });

        repo.get_mut(current).map(|node| {
            node.set_parent(&Some(relative.child)); 
            debug!("\nnode = {:#?}", node);
        });
        debug!("\nend rotate fn");
    }

    fn replace(&mut self, repo: &mut R, removable: &U, replacement: &Option<NodeColor<U>>) {
        let replacement = replacement.as_ref().map(|r| r.id);
        let parent_id = repo.get_parent(removable).map(|p| *p.id());
        let removable_left = parent_id.as_ref().map(|p| {
            if let Some(l) = repo.get_left(p) {
                Self::node_is(removable, &Some(*l.id()))
            } else { false }
        });
        match removable_left {
            None => self.root = replacement.clone(),
            Some(true) => {
                parent_id.as_ref().map(|p| {
                    repo.get_mut(p).unwrap().set_left_child(&replacement);
                });
            }
            Some(false) => {
                parent_id.as_ref().map(|p| {
                    repo.get_mut(&p).unwrap().set_right_child(&replacement);
                });
            }
        }
        if let Some(r) = replacement {
            repo.get_mut(&r).unwrap().set_parent(&parent_id);
        }
    }

    fn node_is(node: &U, other: &Option<U>) -> bool {
        other.map_or(false, |n| n == *node)
    }

    pub fn delete(&mut self, repo: &mut R, value: T) -> Option<U> {
        let mut deleted = self.find_node(repo, value)
            .map(|node| {
                debug!("\nnode for delete = {:#?}", node);
                NodeColor{id: *node.id(), color: *node.color()}
            })?;

        let replaced = match Self::get_if_one_child(repo, &deleted.id) {
            Some(ref child) => {
                self.replace(repo, &deleted.id, &Some(child.clone()));
                Some(child.clone())
            }
            // node has two or no childrens
            None => {
                let right_child = repo.get_right(&deleted.id).map(|n| *n.id());
                match Self::tree_minimum(repo, right_child) {
                    Some(replace_id) => {
                        let replace_key = repo.get(&replace_id).map(|replace| {
                            debug!("\nreplace = {:#?}", replace);
                            deleted.color = *replace.color();
                            replace.key()
                        }).unwrap();
                        repo.get_mut(&deleted.id).map(|node| {
                            node.set_key(replace_key);
                        });
                        let replace_child = repo
                            .get_right(&replace_id)
                            .map(|node| NodeColor{id: *node.id(), color: *node.color()});
                        debug!("\nreplace node child = {:#?}", replace_child);
                        self.replace(repo, &replace_id, &replace_child);
                        replace_child
                    }
                    None => {
                        self.replace(repo, &deleted.id, &None);
                        None
                    }
                }
            }
        };
        if deleted.color == Color::Black {
            self.delete_fixup(repo, &replaced);
        }
        self.dec_len();
        Some(deleted.id)
    }

    fn get_if_one_child(repo: &R, node: &U) -> Option<NodeColor<U>> {
        let child = if repo.get_left(node).is_none() {
            repo.get_right(node)
        } else if repo.get_right(node).is_none() {
            repo.get_left(node)
        } else { None };
        child.map(|node| {
            NodeColor{
                id: *node.id(),
                color: *node.color()
            }
        })
    }

    fn tree_minimum(repo: &R, node: Option<U>) -> Option<U> {
        let mut node = repo.get(&node?);
        let mut result = None;
        while let Some(n) = node {
            result = Some(n.id());
            node = repo.get_left(n.id());
        }
        result.cloned()
    }

    fn delete_fixup(&mut self, repo: &mut R, mut node: &Option<NodeColor<U>>) {
        let black = &Color::Black;
        while let Some(n) = node {
            if n.id != *self.root.as_ref().unwrap()
                && n.color == *black
            {
                let child = &Self::node_is_child(repo, &n.id);
                let parent_id = &repo.get_parent(&n.id).map(|p| p.id());
                let (sibling, rotation) =
                    match child {
                        Some(Child::Left) => {
                            (parent_id.map(|p| repo.get_right(p)).flatten(),
                                Rotation::Left)
                        },
                        Some(Child::Right) => {
                            (parent_id.map(|p| repo.get_left(p)).flatten(),
                                Rotation::Right)
                        },
                        None => { unreachable!() },
                    };
                node = self.delete_fixup_subtree(
                    repo,
                    &n,
                    &sibling.map(|s| *s.id()),
                    &rotation,
                    &child.unwrap(),
                );
            } else { 
                if n.color == !Color::Black {
                    repo.get_mut(&n.id).map(|node| node.set_color(black));
                }
                break;
            }; 
        }
        if let Some(ref r) = self.root {
            repo.get_mut(r).map(|node| node.set_color(black));
        }
    }

    // fn delete_fixup_subtree(&mut self, node: &BareTree<T>, sibling: &Tree<T>,
    //     rotation: &Rotation, node_is_child: &Child) -> Tree<T>
    // {
    //     let node = node.clone();
    //     let mut sibling = sibling.clone();
    //     let mut parent = node.unwrap_parent();
    //     if sibling.is_some() && sibling.as_ref().unwrap().color() == Color::Red {
    //         sibling.as_ref().unwrap().set_color(Color::Black);
    //         parent.set_color(Color::Red);
    //         self.rotate(&mut parent, &rotation.clone());
    //         sibling = match node_is_child {
    //             Child::Left => parent.right_child(),
    //             Child::Right => parent.left_child(),
    //         };
    //     }
    //
    //     let nephews = Self::childrens(&sibling, &node_is_child);
    //     let red_nephews = Self::any_colors(&nephews, &Color::Red);
    //
    //     match red_nephews {
    //         false => {
    //             if sibling.is_some() {
    //                 sibling.unwrap().set_color(Color::Red);
    //             }
    //             node.parent()
    //         }
    //         true => {
    //             let close_nephew = nephews[0].clone();
    //             let distant_nephew = nephews[1].clone();
    //             let distant_black = Self::any_colors(
    //                 &vec![distant_nephew.clone()],
    //                 &Color::Black
    //             );
    //             
    //             if distant_black {
    //                 close_nephew.as_ref().unwrap().set_color(Color::Black);
    //                 sibling.as_ref().unwrap().set_color(Color::Red);
    //                 self.rotate(&mut sibling.unwrap(), &!rotation.clone());
    //                 match node_is_child {
    //                     Child::Left => sibling = node.unwrap_parent().right_child(),
    //                     Child::Right => sibling = node.unwrap_parent().left_child(),
    //                 }
    //             } else if let Some(distant_nephew) = distant_nephew {
    //                 distant_nephew.set_color(Color::Black);
    //             }
    //             sibling.unwrap().set_color(
    //                 node.unwrap_parent().color());
    //             node.unwrap_parent().set_color(Color::Black); 
    //             self.rotate(&mut node.unwrap_parent(), &rotation.clone());
    //             self.root.clone()
    //         }
    //     }
    // }
    //
    // fn childrens(node: &Tree<T>, first_child: &Child) -> Vec<Tree<T>> {
    //     let (left, right) = match node {
    //         Some(n) => (n.left_child(), n.right_child()),
    //         None => (None, None),
    //     };
    //     match first_child {
    //         Child::Left => vec![left, right],
    //         Child::Right => vec![right, left],
    //     }
    // }
    //
    // fn any_colors(nodes: &Vec<Tree<T>>, color: &Color) -> bool {
    //     nodes.iter()
    //         .any(|node| {
    //             if let Some(n) = node {
    //                 n.color() == *color
    //             } else { false }
    //         })
    // }
}
