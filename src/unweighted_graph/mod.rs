use std::{cell::RefCell, rc::Rc};

type Edge = Rc<RefCell<Node>>;

pub struct Node {
    pub data: &'static str,
    pub edges: Vec<Edge>,
}

pub struct Graph {}

impl Node {
    fn first(&self) -> Edge {
        self.edges[0].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rc_is_right() {
        let root = Rc::new(RefCell::new(Node {
            data: "Hello from root",
            edges: vec![],
        }));

        let mut mut_root = root.borrow_mut();

        let second = Rc::new(RefCell::new(Node {
            data: "Hello from second",
            edges: vec![],
        }));

        mut_root.edges.push(second.clone());

        let mut second_mut = second.borrow_mut();

        second_mut.edges.push(root.clone());

        assert_eq!(mut_root.first(), )

        // let mut mut_root = root.borrow_mut();
        // assert_eq!(Rc::strong_count(&root), 1)
    }
}
