use std::{rc::Rc};

use crate::node::Node;

pub struct MyQueue<T: Clone> {
    head: Option<Rc<Node<T>>>,
    tail: Option<Rc<Node<T>>>,
}

impl<T:Clone> MyQueue<T> {
    pub fn new() -> MyQueue<T> {
        MyQueue { head: None, tail: None }
    }

    pub fn push(&mut self, val: T) {
        
    }
    pub fn pop(&mut self)  -> Option<T> {
        None
    }
}

#[cfg(test)]
mod tests {
    fn test_queue() {
        use super::MyQueue;
        let mut queue = MyQueue::new();
        queue.push(1);
        queue.push(2);
        queue.push(3);
        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), None);
    }
}