use std::{
    fs::read_to_string,
};
use serde::{Serialize, Deserialize};
use serde_json::from_str;

#[derive(Deserialize, Serialize, Debug)]
struct Data {
    name: String,
    subjects: Vec<u32>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Node {
    name: u32,
    subjects: Vec<u32>
}

impl Node {
    fn new(name: u32, subjects: Vec<u32>) -> Node {
        Node {
            name,
            subjects
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct Graph {
    total_nodes: u32,
    nodes: Vec<Node>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            total_nodes: 0,
            nodes: Vec::new()
        }
    }

    fn add_node(&mut self, n: Node) {
        self.total_nodes += 1;
        self.nodes.push(n)
    }
}

fn main() {
    let mut graph = Graph::new();

    let json = read_to_string("src/data/data.json").expect("Error");
    let res: Vec<Data> = match from_str(&json) {
        Err(err) => panic!("{}", err),
        Ok(data) => data
    };

    res.into_iter().for_each(|x: Data| {
        let name = x.name.parse().expect("name not number");
        let node = Node::new(name, x.subjects);
        graph.add_node(node);
    });

    println!("{:?}", graph);
}
