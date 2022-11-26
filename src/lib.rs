mod unweighted_graph;
mod weighted_graph;
use unweighted_graph::Node;
// use weighted_graph::Hello;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::*;

    #[test]
    fn do_node() {
        let root = Rc::new(RefCell::new(Node {
            data: "23",
            edges: vec![],
        }));

        let b = Rc::new(RefCell::new(Node {
            data: "28",
            edges: vec![],
        }));

        let mut mut_root = root.borrow_mut();

        mut_root.edges.push(b.clone());
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
