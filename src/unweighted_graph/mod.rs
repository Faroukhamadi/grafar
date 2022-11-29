use std::{cell::RefCell, rc::Rc, sync::Arc};

type Edge = Rc<RefCell<Node>>;

#[derive(Debug)]
pub struct Node {
    pub data: i32,
    pub edges: Vec<Edge>,
}

impl From<Vec<Vec<i32>>> for Graph {
    fn from(adjacency_matrix: Vec<Vec<i32>>) -> Graph {
        let mut g = Graph { nodes: vec![] };
        for _ in 0..adjacency_matrix.len() {
            let node = Node::new(0, vec![]);
            g.nodes.push(node);
        }
        println!("we are fine here");
        // for i in 0..adjacency_matrix.len() {
        //     for j in 0..adjacency_matrix[i].len() {
        //         if adjacency_matrix[i][j] == 1 {
        //             g.nodes[i].borrow_mut().edges.push(g.nodes[j].clone());
        //         }
        //     }
        // }
        g
    }
}

pub struct Graph {
    pub nodes: Vec<Edge>,
}

impl Node {
    fn first(&self) -> Edge {
        self.edges[0].clone()
    }
    fn new(data: i32, edges: Vec<Edge>) -> Edge {
        Rc::new(RefCell::new(Node { data, edges }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_graph_from_adjacency_matrix() {
        let g = Graph::from(vec![
            vec![0, 0, 1, 1],
            vec![0, 0, 1, 0],
            vec![1, 1, 0, 1],
            vec![1, 0, 1, 1],
        ]);

        println!("{:?}", g.nodes);
    }

    #[test]
    fn creates_graph() {
        let mut g = Graph { nodes: vec![] };
        let root = Node::new(1, vec![]);

        g.nodes.push(root);
    }

    #[test]
    fn first_returns_node() {
        let root = Node::new(1, vec![]);
        let second = Node::new(2, vec![]);

        let mut mut_root = root.borrow_mut();

        mut_root.edges.push(second);

        assert_eq!(mut_root.first().borrow().data, 2);
    }

    #[test]
    fn rc_is_right() {
        let root = Node::new(1, vec![]);
        let mut mut_root = root.borrow_mut();

        let second = Node::new(2, vec![]);
        mut_root.edges.push(second.clone());

        assert_eq!(Rc::strong_count(&root), 1)
    }
}
