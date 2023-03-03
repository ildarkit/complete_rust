mod rb_tree;

#[cfg(test)]
mod tests {
    use super::rb_tree::*;

    #[test]
    fn not_found_test() {
        let mut rb_tree = RedBlackTree::new();
        assert!(rb_tree.find(0).is_none());
        rb_tree.insert(1);
        assert!(rb_tree.find(2).is_none());
    }

    #[test]
    fn found_test() {
        let mut rb_tree = RedBlackTree::new();
        rb_tree.insert(0);
        let node = rb_tree.find(0).unwrap();
        assert_eq!(node.key, 0);
        assert_eq!(node.color, Color::Black);
        for i in 1..=100 {
            rb_tree.insert(i);
        }
        for i in 0..=100 {
            let node = rb_tree.find(i).unwrap();
            assert_eq!(node.key, i);
        }
    }
}
