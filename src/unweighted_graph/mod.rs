use std::{cell::RefCell, rc::Rc};

pub struct Node {
    pub data: &'static str,
    pub edges: Vec<Rc<RefCell<Node>>>,
}

pub struct Graph {}
