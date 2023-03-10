// example implementation of BFS in rust
// from: https://www.sotr.blog/articles/breadth-first-search

#![allow(dead_code)]

use std::collections::VecDeque;

struct Queue<T> {
    pub items: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            items: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, v: T) {
        self.items.push_back(v);
    }

    pub fn dequeue(&mut self) -> T {
        self.items
            .pop_front()
            .expect("Cannot dequeue from an empty queue")
    }

    pub fn is_empty(&self) -> bool {
        self.items.len() == 0
    }
}


type Vertex = Vec<u32>;
type Graph = Vec<Vertex>;


fn bfs(graph: Graph, start_node: u32, end_node: u32) -> Option<Vec<Option<u32>>> {
    let mut queue = Queue::new();
    queue.enqueue(start_node);

    let mut visited_vertices = vec![false; graph.len()];
    visited_vertices[0] = true;

    let mut prev: Vec<Option<u32>> = vec![None; graph.len()];

    'outer: while !queue.is_empty() {
        let current_node = queue.dequeue();
        for v in graph[current_node as usize].iter() {
            // if we reach the desired node, exit the loop
            if *v == end_node {
                prev[*v as usize] = Some(current_node);
                break 'outer;
            }

            // if not desired node, queue it and set as visited
            if !visited_vertices[*v as usize] {
                queue.enqueue(*v);
                visited_vertices[*v as usize] = true;
                prev[*v as usize] = Some(current_node);
            }
        }
    }

    let mut path = Vec::new();
    let mut at = Some(end_node);

    while at != None {
        path.push(at);
        at = prev[at.unwrap_or(0) as usize];
    }

    path.reverse();

    return match path[0] {
        Some(x) if x == start_node => Some(path),
        _ => None,
    };

}


#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn existing_path() {
        let graph: Graph = vec![
            vec![1, 2],
            vec![0, 3, 4, 1],
            vec![0, 4],
            vec![1, 4, 5],
            vec![1, 2, 3, 5],
            vec![3, 4, 6],
            vec![7, 5],
            vec![6],
        ];

        assert_eq!(
            bfs(graph, 0, 7).unwrap(),
            vec![Some (0), Some(1), Some(3), Some(5), Some(6), Some(7)]
        )
    }
 
    #[test]
    fn no_existing_path() {
        let graph: Graph = vec![
            vec![1, 2, 5],
            vec![0, 1, 3, 4],
            vec![0, 3],
            vec![1, 4, 5, 2],
            vec![1, 3, 5],
            vec![0, 3, 4, 1],
            vec![7],
            vec![6],
        ];

        assert_eq!(
            bfs(graph, 0, 7),
            None
        )
    }
}



