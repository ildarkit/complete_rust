pub(crate) type Tree<T> = Box<Node<T>>;
pub(crate) type Data<T> = (Option<T>, Option<Tree<T>>);

pub trait Identity {
    fn id(&self) -> usize;
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

#[derive(Clone, Default)]
pub(crate) struct Node<T> {
    values: Vec<Option<T>>,
    children: Vec<Option<Tree<T>>>,
    left_child: Option<Tree<T>>,
    node_type: NodeType,
}

impl<T> Node<T>
    where
        T: Clone + Default + Identity
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

    pub(crate) fn children(&self) -> &[Option<Tree<T>>] {
        &self.children[..]
    }

    pub(crate) fn len(&self) -> usize {
        self.children.len() + 1
    }

    pub(crate) fn get_type(&self) -> NodeType {
        self.node_type.clone()
    }

    pub(crate) fn left_child(&self) -> &Option<Tree<T>> {
        &self.left_child
    }

    pub(crate) fn split(&mut self) -> (T, Tree<T>) {
        let mut sibling = Node::new(self.node_type.clone());
        let val_count = self.values.len();
        let split_at = val_count / 2usize;
        let val = self.values.remove(split_at);
        let node = self.children.remove(split_at);

        for _ in split_at..self.values.len() {
            let value = self.values.pop().unwrap();
            let child = self.children.pop().unwrap();
            sibling.add_key(value.as_ref().unwrap().id(),
                (value, child)
            );
        }
        sibling.add_left_child(node);
        (val.unwrap(), Box::new(sibling))
    }

    pub(crate) fn get_value(&self, key: usize) -> Option<&T> {
        let mut result = None;
        for v in self.values.iter() {
            if let Some(value) = v {
                if value.id() == key {
                    result = Some(value);
                    break;
                }
            }
        }
        result
    }

    pub(crate) fn add_left_child(&mut self, node: Option<Tree<T>>) {
        self.left_child = node;
    }

    pub(crate) fn add_key(&mut self, key: usize, value: Data<T>) {
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

    pub(crate) fn remove_key(&mut self, id: usize) -> Option<(usize, Data<T>)> {
        match self.find_closest_index(id) {
            Direction::Left => {
                let tree = self.left_child.take();
                Some((id, (None, tree)))
            }
            Direction::Right(index) => {
                let val = self.values.remove(index);
                let tree = self.children.remove(index);
                Some((val.as_ref().unwrap().id(), (val, tree)))
            }
        }
    }

    pub(crate) fn find_closest_index(&self, key: usize) -> Direction {
        let mut index = Direction::Left;
        for (i, pair) in self.values.iter().enumerate() {
            if let Some(val) = pair {
                if val.id() <= key {
                    index = Direction::Right(i);
                } else { break }
            }
        }
        index
    }

    pub(crate) fn get_child(&self, key: usize) -> Option<&Tree<T>> {
        match self.find_closest_index(key) {
            Direction::Left => self.left_child.as_ref(),
            Direction::Right(i) => self.children[i].as_ref(),
        }
    }
}
