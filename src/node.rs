use std::rc::Rc;

pub struct Node<T: Clone> {
    val: T,
    next: Option<Rc<Node<T>>>,
}

impl<T: Clone> Node<T> {
    pub fn new(val: T) -> Node<T> {
        Node {val: val, next: None}
    }

    pub fn new_with_next(val: T, next: Rc<Node<T>>) -> Node<T> {
        Node {val: val, next: Some(next)}
    }

    pub fn set_next(&mut self, next: Rc<Node<T>>) {
        self.next = Some(next);
    }

    pub fn get_next(&self) -> Option<Rc<Node<T>>> {
        match &self.next {
            None => None,
            Some(node) => Some(Rc::clone(node)),
        }
    }

    pub fn get_val(&self) -> &T {
        &self.val
    }
}