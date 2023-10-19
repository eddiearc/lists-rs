use std::rc::Rc;
use crate::node::Node;

pub struct Iterator<T: Clone> {
    head: Option<Rc<Node<T>>>,
}