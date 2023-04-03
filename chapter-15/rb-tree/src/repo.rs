use std::fmt;
use crate::node::Operations;

pub trait Repository<T, U> 
    where 
        T: Default + fmt::Debug,
        U: Default + fmt::Debug
{
    type Output: Operations<T, U>;

    fn new() -> Self;

    fn add(&mut self, id: &U, key: T);

    fn remove(&mut self, node_id: &U) -> Option<Self::Output>;

    fn get(&self, node_id: &U) -> Option<&Self::Output>;

    fn get_mut(&mut self, node_id: &U) -> Option<&mut Self::Output>;

    fn mut_parent(&mut self, node: &U) -> Option<&mut Self::Output> {
        self.get(node)
            .map(|n| n.parent())
            .flatten()
            .as_ref()
            .map(|p| self.get_mut(p))
            .flatten()
    }

    fn get_parent(&self, node: &U) -> Option<&Self::Output> {
        self.get(node)
            .map(|n| n.parent())
            .flatten()
            .as_ref()
            .map(|p| self.get(p))
            .flatten()
    }
    
    fn mut_left(&mut self, node: &U) -> Option<&mut Self::Output> {
        self.get(node)
            .map(|n| n.left_child())
            .flatten()
            .as_ref()
            .map(|n| self.get_mut(n))
            .flatten()
    }

    fn get_left(&self, node: &U) -> Option<&Self::Output> {
        self.get(node)
            .map(|n| n.left_child())
            .flatten()
            .as_ref()
            .map(|n| self.get(n))
            .flatten()
    }

    fn mut_right(&mut self, node: &U) -> Option<&mut Self::Output> {
        self.get(node)
            .map(|n| n.right_child())
            .flatten()
            .as_ref()
            .map(|n| self.get_mut(n))
            .flatten()
    }

    fn get_right(&self, node: &U) -> Option<&Self::Output> {
        self.get(node)
            .map(|n| n.right_child())
            .flatten()
            .as_ref()
            .map(|n| self.get(n))
            .flatten()
    }
}
