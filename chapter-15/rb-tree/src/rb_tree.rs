use std::fmt;
use std::marker::PhantomData;
use std::hash::Hash;
use std::ops::{Not, AddAssign};
use log::debug;

use crate::node::{Color, Operations};
use crate::repo::Repository;

type DefaultId = u32;

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
        R::Output: Operations<T, U> + fmt::Debug + PartialOrd,
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
}
