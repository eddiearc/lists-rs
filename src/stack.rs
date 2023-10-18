use std::{cell::RefCell, rc::Rc};

pub struct MyStack<T> {
    cur_node: Option<RefCell<Rc<Node<T>>>>,
}

impl<T> MyStack<T> {
    pub fn push(&mut self, val: T) {
        
    }

    pub fn pop(&mut self) -> Option<T> {
        None
    }
}

// Node for MyStack
struct Node<T> {
    val: T,
    prev: Option<RefCell<Rc<Node<T>>>>,
    next: Option<RefCell<Rc<Node<T>>>>,
}