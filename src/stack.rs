use std::{rc::Rc};

pub struct MyStack<T: Clone> {
    tail: Option<Rc<Node<T>>>,
}

impl<T: Clone> MyStack<T> {
    pub fn new() -> MyStack<T> {
        return MyStack { tail: None }
    }
    pub fn push(&mut self, val: T) {
        match self.tail {
            None => {
                let node = Rc::new(Node::new(val));
                self.tail = Some(node);
            },
            Some(ref mut node) => {
                let mut new_node = Node::new(val);
                new_node.next = Some(Rc::clone(node));
                self.tail = Some(Rc::new(new_node));
            },
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.tail.is_none() {
            return None;
        }
        let binding = self.tail.take();
        let node = binding.as_ref().unwrap();
        self.tail = match &node.next {
            None => None,
            Some(node) => Some(Rc::clone(&node)),
        };
        
        return Some(node.val.clone());
    }
}

// Node for MyStack
struct Node<T: Clone> {
    val: T,
    next: Option<Rc<Node<T>>>,
}

impl<T: Clone> Node<T> {
    fn new(val: T) -> Node<T> {
        Node {val: val, next: None}
    }
}
