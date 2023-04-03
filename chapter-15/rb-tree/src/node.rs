use std::fmt;
use std::ops::Not;
use std::cmp::Ordering;

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub enum Color {
    #[default]
    Red,
    Black,
}

impl Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
        }
    }
}

pub trait Operations<T, U> {
    fn new(id: &U, key: T) -> Self;

    fn key(&self) -> T;

    fn set_key(&mut self, key: &T);

    fn set_left_child(&mut self, node: &Option<U>);

    fn set_right_child(&mut self, node: &Option<U>);

    fn set_parent(&mut self, node: &Option<U>);

    fn left_child(&self) -> Option<U>;

    fn right_child(&self) -> Option<U>;

    fn parent(&self) -> Option<U>;

    fn id(&self) -> &U;

    fn color(&self) -> &Color;

    fn set_color(&mut self, color: &Color);
}

#[derive(Debug, Default, Clone)]
pub struct Node<T, U> 
   where
        T: Default + fmt::Debug,
        U: Default + fmt::Debug
{
    id: U,
    color: Color,
    key: T,
    parent: Option<U>,
    left: Option<U>,
    right: Option<U>,
}

impl<T, U> Operations<T, U> for Node<T, U> 
    where
        T: Default + Clone + fmt::Debug,
        U: PartialEq + PartialOrd + Default + Copy + fmt::Debug
{
    fn new(id: &U, key: T) -> Self {
        Self {
            id: *id,
            key,
            ..Default::default()
        }
    }

    fn key(&self) -> T {
        self.key.clone()
    }

    fn set_key(&mut self, key: &T) {
        self.key = key.clone();
    }

    fn set_left_child(&mut self, node: &Option<U>) {
        self.left = node.clone();
    }

    fn set_right_child(&mut self, node: &Option<U>) {
        self.right = node.clone();
    }

    fn set_parent(&mut self, node: &Option<U>) {
        self.parent = node.clone();
    }

    fn left_child(&self) -> Option<U> {
        self.left.clone()
    }

    fn right_child(&self) -> Option<U> {
        self.right.clone()
    } 

    fn parent(&self) -> Option<U> {
        self.parent.clone()
    }

    fn id(&self) -> &U {
        &self.id
    }

    fn color(&self) -> &Color {
        &self.color
    }

    fn set_color(&mut self, color: &Color) {
        self.color = *color;
    }
} 

impl<T, U> PartialEq for Node<T, U> 
    where 
        T: PartialEq + Default + Copy + Clone + fmt::Debug,
        U: Default + Copy + fmt::Debug
{
    fn eq(&self, other: &Node<T, U>) -> bool {
        self.key == other.key
    }
}

impl<T, U> PartialOrd for Node<T, U>
    where
        T: PartialOrd + Default + Copy + Clone + fmt::Debug,
        U: Default + Copy + fmt::Debug
{
    fn partial_cmp(&self, other: &Node<T, U>) -> Option<Ordering> {
        self.key.partial_cmp(&other.key)
    }
}
