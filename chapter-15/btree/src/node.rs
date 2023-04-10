use std::marker::PhantomData;

pub(crate) type Tree<U, T> = Box<Node<U, T>>;
pub(crate) type Data<U, T> = (Option<T>, Option<Tree<U, T>>);

pub trait Key<U: Copy> {
    fn key(&self) -> U;
}

#[derive(Clone, PartialEq, Debug, Default)]
pub(crate) enum NodeType {
    #[default]
    Leaf,
    Regular,
}

#[derive(Clone, PartialEq)]
pub(crate) enum Direction {
    Left,
    Right(usize),
}

#[derive(Clone, Default, Debug)]
pub(crate) struct Node<U, T> {
    values: Vec<Option<T>>,
    children: Vec<Option<Tree<U, T>>>,
    left_child: Option<Tree<U, T>>,
    node_type: NodeType,
    phantom: PhantomData<U>,
}

impl<U, T> Node<U, T>
    where
        T: Clone + Default + Key<U>,
        U: Copy + Default + PartialEq + PartialOrd,
{

    pub(crate) fn new_leaf() -> Self {
       Self::new(NodeType::Leaf) 
    }

    pub(crate) fn new_regular() -> Self {
        Self::new(NodeType::Regular)
    }

    pub(crate) fn new(node_type :NodeType) -> Self {
        Self {
            node_type,
            ..Default::default() 
        }
    }

    pub(crate) fn values_len(&self) -> usize {
        self.values.len()
    }

    pub(crate) fn children(&self) -> &[Option<Tree<U, T>>] {
        &self.children[..]
    }

    pub(crate) fn values(&self) -> &[Option<T>] {
        &self.values[..]
    }

    pub(crate) fn len(&self) -> usize {
        self.children.len() + 1
    }

    pub(crate) fn get_type(&self) -> NodeType {
        self.node_type.clone()
    }

    pub(crate) fn left_child(&self) -> &Option<Tree<U, T>> {
        &self.left_child
    }

    pub(crate) fn split(&mut self) -> (T, Tree<U, T>) {
        let mut sibling = Node::new(self.node_type.clone());
        let val_count = self.values.len();
        let split_at = val_count / 2usize;
        let val = self.values.remove(split_at);
        let node = self.children.remove(split_at);

        for _ in split_at..self.values.len() {
            let value = self.values.pop().unwrap();
            let child = self.children.pop().unwrap();
            sibling.add_key(value.as_ref().unwrap().key(),
                (value, child)
            );
        }
        sibling.add_left_child(node);
        (val.unwrap(), Box::new(sibling))
    }

    pub(crate) fn get_value(&self, key: U) -> Option<&T> {
        let mut result = None;
        for v in self.values.iter() {
            if let Some(value) = v {
                if value.key() == key {
                    result = Some(value);
                    break;
                }
            }
        }
        result
    }

    pub(crate) fn add_left_child(&mut self, node: Option<Tree<U, T>>) {
        self.left_child = node;
    }

    pub(crate) fn add_key(&mut self, key: U, value: Data<U, T>) {
        let pos = match self.find_closest_index(key) {
            Direction::Left => 0,
            Direction::Right(p) => p + 1,
        };
        let (val, tree) = value;
        if pos >= self.values.len() {
            self.values.push(val);
            self.children.push(tree);
        } else {
            self.values.insert(pos, val);
            self.children.insert(pos, tree);
        }
    }

    pub(crate) fn remove_key(&mut self, key: U) -> Option<(U, Data<U, T>)> {
        match self.find_closest_index(key) {
            Direction::Left => {
                let tree = self.left_child.take();
                Some((key, (None, tree)))
            }
            Direction::Right(index) => {
                let val = self.values.remove(index);
                let tree = self.children.remove(index);
                Some((val.as_ref().unwrap().key(), (val, tree)))
            }
        }
    }

    pub(crate) fn find_closest_index(&self, key: U) -> Direction {
        let mut index = Direction::Left;
        for (i, pair) in self.values.iter().enumerate() {
            if let Some(val) = pair {
                if val.key() <= key {
                    index = Direction::Right(i);
                } else { break }
            }
        }
        index
    }

    pub(crate) fn get_child(&self, key: U) -> Option<&Tree<U, T>> {
        match self.find_closest_index(key) {
            Direction::Left => self.left_child.as_ref(),
            Direction::Right(i) => self.children[i].as_ref(),
        }
    }
}
