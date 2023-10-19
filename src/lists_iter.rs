use std::rc::Rc;
use crate::node::Node;

pub struct MyIterator<T: Clone> {
    #[allow(dead_code)]
    head: Option<Rc<Node<T>>>,
}

impl<T: Clone> MyIterator<T> {
    pub fn new(head: Rc<Node<T>>) -> MyIterator<T> {
        MyIterator { head: Some(head) }
    }
}

impl <T: Clone> std::iter::Iterator for MyIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.head {
            None => None,
            Some(node) => {
                let val = node.get_val().clone();
                self.head = node.get_next();
                Some(val)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::MyIterator;
    use super::Node;

    #[test]
    fn next_one() {
        let head = Rc::new(Node::new(1));
        let mut iter = MyIterator::new(Rc::clone(&head));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn next_more() {
        let head = Node::new_with_next(1, 
            Rc::new(Node::new_with_next(2, 
                Rc::new(Node::new(3)))));
        
        let mut iter = MyIterator::new(Rc::new(head));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }
}