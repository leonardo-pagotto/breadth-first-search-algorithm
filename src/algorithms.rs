use std::collections::{HashMap, VecDeque};
use crate::model::GraphAdj;

pub fn bfs(graph: &GraphAdj, start: usize, end: usize) -> Option<Vec<usize>> {

    let graph_len = graph.data.len();

    if start >= graph_len || end >= graph_len {
        return None;
    }

    if start == end {
        return Some(vec![start]);
    }

    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut visited: Vec<bool> = vec![false; graph_len];
    let mut parent: HashMap<usize, usize> = HashMap::new();

    visited[start] = true;
    queue.push_back(start);
    
    while let Some(node) = queue.pop_front() {
        let neighbors = &graph.data[node];

        for &neighbor in neighbors {
            if visited[neighbor] { continue; }

            visited[neighbor] = true;
            parent.insert(neighbor, node);

            if neighbor != end {
                queue.push_back(neighbor);
                continue;
            }

            let mut path: Vec<usize> = vec![];
            let mut current = end;

            while current != start {
                path.push(current);
                current = parent[&current];
            }

            path.push(start);
            path.reverse();

            return Some(path);
        }
    }

    None
}