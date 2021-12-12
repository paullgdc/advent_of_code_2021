use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
    str::FromStr,
};

use arrayvec::ArrayStr;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Node {
    Start,
    End,
    Big(ArrayStr<2>),
    Small(ArrayStr<2>),
}

impl Node {
    fn from_str(s: &str) -> Self {
        match s {
            "start" => Self::Start,
            "end" => Self::End,
            _ if s.chars().all(char::is_lowercase) => Self::Small(ArrayStr::from_str(s).unwrap()),
            _ if s.chars().all(char::is_uppercase) => Self::Big(ArrayStr::from_str(s).unwrap()),
            _ => panic!(),
        }
    }
}

fn parse_input() -> Vec<(Node, Node)> {
    std::io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|l| {
            let (left, right) = l.trim().split_once('-').unwrap();

            (Node::from_str(left), Node::from_str(right))
        })
        .collect()
}

fn adajacency_matrix(edges: &[(Node, Node)]) -> HashMap<Node, Vec<Node>> {
    let mut matrix: HashMap<Node, Vec<Node>> = HashMap::new();
    for (start, finish) in edges {
        matrix.entry(start.clone()).or_default().push(finish.clone());
        matrix.entry(finish.clone()).or_default().push(start.clone());
    }
    matrix
}

fn path_next_child(path: &mut Vec<(Node, usize)>) {
    path.last_mut().map(|(_, i)| *i += 1);
}

fn step_1(edges: &[(Node, Node)]) {
    let adjacent = adajacency_matrix(edges);
    let mut path = Vec::new();
    let mut encountered = HashSet::new();

    path.push((Node::Start, 0));
    encountered.insert(Node::Start);

    let mut n_paths = 0;

    loop {
        let (current_node, next_child) = match path.last() {
            Some(n) => n.clone(),
            None => break,
        };
        let neighs = &adjacent[&current_node];
        if neighs.len() <= next_child {
            let (node, _) = path.pop().unwrap();
            encountered.remove(&node);
            path.last_mut().map(|(_, idx)| *idx += 1);
            continue;
        }

        let next_node = neighs[next_child].clone();

        match next_node {
            Node::Start => {
                path_next_child(&mut path);
                continue;
            }
            Node::End => {
                path_next_child(&mut path);
                n_paths += 1;
                continue;
            }
            Node::Small(_) => {
                if encountered.contains(&next_node) {
                    path_next_child(&mut path);
                    continue;
                }
            }
            _ => {}
        }

        path.push((next_node.clone(), 0));
        encountered.insert(next_node);
    }

    println!("First step solution {}", n_paths);
}

fn step_2(edges: &[(Node, Node)]) {
    let adjacent = adajacency_matrix(edges);
    let mut path = Vec::new();
    let mut encountered = HashSet::new();

    path.push((Node::Start, 0));
    encountered.insert(Node::Start);

    let mut n_paths = 0;
    let mut visited_twice = None;

    loop {
        let (current_node, next_child) = match path.last() {
            Some(n) => n.clone(),
            None => break,
        };
        let neighs = &adjacent[&current_node];
        if neighs.len() <= next_child {
            let (node, _) = path.pop().unwrap();
            if visited_twice.as_ref() == Some(&node) {
                visited_twice = None;
            } else {
                encountered.remove(&node);
            }
            path.last_mut().map(|(_, idx)| *idx += 1);
            continue;
        }

        let next_node = neighs[next_child].clone();

        match next_node {
            Node::Start => {
                path_next_child(&mut path);
                continue;
            }
            Node::End => {
                path_next_child(&mut path);
                n_paths += 1;
                continue;
            }
            Node::Small(_) => {
                if encountered.contains(&next_node) {
                    if let None = visited_twice {
                        visited_twice = Some(next_node.clone());
                    } else {
                        path_next_child(&mut path);
                        continue;
                    }
                }
            }
            _ => {}
        }

        path.push((next_node.clone(), 0));
        encountered.insert(next_node);
    }

    println!("Second step solution {}", n_paths);
}

fn main() {
    let edges = parse_input();
    step_1(&edges);
    step_2(&edges);
}
