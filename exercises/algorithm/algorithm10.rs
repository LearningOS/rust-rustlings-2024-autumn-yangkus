use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct NodeNotInGraph;

impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }

    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }

    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }

    fn add_node(&mut self, node: &str) -> bool {
        if self.adjacency_table.contains_key(node) {
            false // 节点已经存在
        } else {
            self.adjacency_table.insert(node.to_string(), Vec::new());
            true // 成功添加节点
        }
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (from, to, weight) = edge;

        // 确保这两个节点存在
        self.add_node(from);
        self.add_node(to);

        // 添加边
        self.adjacency_table
            .entry(from.to_string())
            .or_insert_with(Vec::new)
            .push((to.to_string(), weight));
        
        self.adjacency_table
            .entry(to.to_string())
            .or_insert_with(Vec::new)
            .push((from.to_string(), weight)); // 无向图：相互添加
    }
}

pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;

    fn add_node(&mut self, node: &str) -> bool;

    fn add_edge(&mut self, edge: (&str, &str, i32));

    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }

    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }

    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        let mut seen = HashSet::new(); // 用于跟踪已添加的边
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                // 确保只添加一次边（避免重复边的情况）
                if !seen.contains(&(from_node, to_node)) {
                    edges.push((from_node, to_node, *weight));
                    seen.insert((from_node, to_node)); // 记录已添加的边
                }
            }
        }
        edges
    }
}

#[cfg(test)]
mod test_undirected_graph {
    use super::{Graph, UndirectedGraph};

    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        // 这里修正 expected_edges 的构造
        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];

        for edge in expected_edges.iter() {
            assert!(graph.edges().contains(edge), "Expected edge {:?} not found in graph edges", edge);
        }
    }
}
