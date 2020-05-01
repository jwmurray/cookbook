/// Imagine a group of nodes
/// a,b,c,d,e
/// where
///     w(a,b) = 6
///     w(a,d) = 1
///     w(d,b) = 2
///     w(b,e) = 2
///     w(d,e) = 1
///     w(b,c) = 5
///     w(e,c) = 5
///     w(b,e) = 2
///
/// Find the dijkstra shortest path
///
/// Data Structures:
///
/// vector of visited
/// vector of not visited
/// starting point
/// vector of distances from starting point to all nodes
use std::char;
use std::collections::HashMap;

fn main() {
    let nodes = (10..10 + 5)
        .map(|i| char::from_digit(i, 36).unwrap())
        .collect::<Vec<_>>();
    let mut edges = HashMap::new();
    edges.insert(('a', 'b'), 6);
    edges.insert(('a', 'd'), 1);
    edges.insert(('d', 'b'), 2);
    edges.insert(('b', 'e'), 2);
    edges.insert(('d', 'e'), 1);
    edges.insert(('b', 'c'), 5);
    edges.insert(('e', 'c'), 5);
    edges.insert(('b', 'e'), 2);
    // let nodes: Vec<u32> = ('a'..'f').collect();
    for item in &nodes {
        println!("item: {}", item);
    }
    println!("{:?}", nodes);
    println!("{:?}", edges);
    println!("weight: {}", edges[&('b', 'e')]);
    for edge in edges {
        println!("Edge: {:?}", edge);
    }
}
