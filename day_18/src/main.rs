#![feature(let_else)]
#![feature(bool_to_option)]

use core::panic;
use std::{io::BufRead, mem};

type NodeIdx = u16;

struct Node {
    parent: Option<NodeIdx>,
    content: NodeType,
}

#[derive(Clone, Copy)]
enum NodeType {
    Leaf(u8),
    Pair { left: NodeIdx, right: NodeIdx },
}

impl NodeType {
    fn leaf_mut(&mut self) -> Option<&mut u8> {
        match self {
            Self::Pair { left: _, right: _ } => None,
            Self::Leaf(leaf) => Some(leaf),
        }
    }
}

struct Arena<T> {
    storage: Vec<Option<T>>,
    free_list: Vec<NodeIdx>,
}

impl<T> Arena<T> {
    fn add(&mut self, e: T) -> NodeIdx {
        match self.free_list.pop() {
            Some(idx) => {
                assert!(self.storage[idx as usize].is_none());
                self.storage[idx as usize] = Some(e);
                idx
            }
            None => {
                self.storage.push(Some(e));
                (self.storage.len() - 1) as NodeIdx
            }
        }
    }

    fn remove(&mut self, idx: NodeIdx) -> T {
        let e = self.storage.get_mut(idx as usize).unwrap().take().unwrap();
        self.free_list.push(idx);
        e
    }

    fn get(&self, idx: NodeIdx) -> &T {
        self.storage[idx as usize].as_ref().unwrap()
    }

    fn get_mut(&mut self, idx: NodeIdx) -> &mut T {
        self.storage[idx as usize].as_mut().unwrap()
    }
}

fn new_subtree_arena(a: &Arena<Node>, roots: &[NodeIdx]) -> (Arena<Node>, Vec<NodeIdx>) {
    let mut new_arena = Arena {
        storage: Vec::new(),
        free_list: Vec::new(),
    };
    fn copy_tree(old: &Arena<Node>, new: &mut Arena<Node>, root: NodeIdx) -> NodeIdx {
        match old.get(root).content {
            NodeType::Leaf(v) => {
                return new.add(Node {
                    parent: None,
                    content: NodeType::Leaf(v),
                })
            }
            NodeType::Pair { left, right } => {
                let (left, right) = (copy_tree(old, new, left), copy_tree(old, new, right));
                let new_node = new.add(Node {
                    parent: None,
                    content: NodeType::Pair { left, right },
                });
                new.get_mut(left).parent = Some(new_node);
                new.get_mut(right).parent = Some(new_node);
                new_node
            }
        }
    }
    let new_roots = roots
        .iter()
        .copied()
        .map(|r| copy_tree(a, &mut new_arena, r))
        .collect();
    (new_arena, new_roots)
}

fn take_num(s: &str) -> (&str, &str) {
    let sep = s.find(|c| !char::is_numeric(c)).unwrap();
    s.split_at(sep)
}

fn parse_input() -> (Arena<Node>, Vec<NodeIdx>) {
    let mut arena = Arena {
        storage: Vec::new(),
        free_list: Vec::new(),
    };
    let trees = std::io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|l| {
            fn parse_node<'a>(a: &mut Arena<Node>, s: &'a str) -> (NodeIdx, &'a str) {
                if s.chars().next().unwrap() == '[' {
                    let (node, rest) = parse_pair(a, s);
                    (node, rest)
                } else {
                    let (num, left) = take_num(s);
                    let node = a.add(Node {
                        parent: None,
                        content: NodeType::Leaf(num.parse().unwrap()),
                    });
                    (node, left)
                }
            }
            fn parse_pair<'a>(a: &mut Arena<Node>, s: &'a str) -> (NodeIdx, &'a str) {
                let pair = s.strip_prefix('[').unwrap();
                let (left, rest) = parse_node(a, pair);
                let rest = rest.strip_prefix(',').unwrap();
                let (right, rest) = parse_node(a, rest);

                let node = a.add(Node {
                    parent: None,
                    content: NodeType::Pair { left, right },
                });
                a.get_mut(left).parent = Some(node);
                a.get_mut(right).parent = Some(node);
                (node, rest.strip_prefix(']').unwrap())
            }
            parse_node(&mut arena, &l).0
        })
        .collect();
    (arena, trees)
}

fn add_trees(a: &mut Arena<Node>, left: NodeIdx, right: NodeIdx) -> NodeIdx {
    assert!(a.get(left).parent.is_none());
    assert!(a.get(right).parent.is_none());
    let idx = a.add(Node {
        parent: None,
        content: NodeType::Pair { left, right },
    });
    a.get_mut(left).parent = Some(idx);
    a.get_mut(right).parent = Some(idx);
    idx
}

fn find_lefmost_4_deep(a: &Arena<Node>, root: NodeIdx) -> Option<NodeIdx> {
    fn _rec(a: &Arena<Node>, root: NodeIdx, depth: u8) -> Option<NodeIdx> {
        match a.get(root).content {
            NodeType::Leaf(_) => None,
            NodeType::Pair { left, right } => {
                if depth == 4 {
                    return Some(root);
                }
                _rec(a, left, depth + 1).or_else(|| _rec(a, right, depth + 1))
            }
        }
    }
    _rec(a, root, 0)
}

fn find_lefmost_bigger_10(a: &Arena<Node>, root: NodeIdx) -> Option<NodeIdx> {
    fn _rec(a: &Arena<Node>, root: NodeIdx) -> Option<NodeIdx> {
        match a.get(root).content {
            NodeType::Leaf(v) => (v >= 10).then_some(root),
            NodeType::Pair { left, right } => _rec(a, left).or_else(|| _rec(a, right)),
        }
    }
    _rec(a, root)
}

enum Direction {
    Left,
    Right,
}

fn find_next_leaf(a: &Arena<Node>, start: NodeIdx, direction: Direction) -> Option<NodeIdx> {
    let mut go_up = if let NodeType::Leaf(_) = a.get(start).content {
        true
    } else {
        false
    };
    let mut curr = start;
    loop {
        if go_up {
            let parent = a.get(curr).parent?;
            let NodeType::Pair {left, right} = &a.get(parent).content else {
                panic!();
            };

            let next_node = *match &direction {
                Direction::Left => right,
                Direction::Right => left,
            };
            if next_node != curr {
                go_up = false;
                curr = next_node;
            } else {
                curr = parent
            }
        } else {
            match &a.get(curr).content {
                NodeType::Leaf(_) => return Some(curr),
                NodeType::Pair { left, right } => {
                    curr = *match &direction {
                        Direction::Left => left,
                        Direction::Right => right,
                    };
                }
            }
        }
    }
}

fn explode_pair(a: &mut Arena<Node>, pair_to_explode: NodeIdx) {
    let pair = mem::replace(&mut a.get_mut(pair_to_explode).content, NodeType::Leaf(0));
    let NodeType::Pair{left, right} = pair else {
        panic!();
    };
    let NodeType::Leaf(left) = a.remove(left).content else {
        panic!();
    };
    let NodeType::Leaf(right) = a.remove(right).content else {
        panic!();
    };
    if let Some(next_left) = find_next_leaf(a, pair_to_explode, Direction::Left) {
        *a.get_mut(next_left).content.leaf_mut().unwrap() += right;
    }
    if let Some(next_right) = find_next_leaf(a, pair_to_explode, Direction::Right) {
        *a.get_mut(next_right).content.leaf_mut().unwrap() += left;
    }
}

fn split_leaf(a: &mut Arena<Node>, to_split: NodeIdx) {
    let v = *a.get_mut(to_split).content.leaf_mut().unwrap();
    let left = v / 2;
    let right = v - left;
    let left = a.add(Node {
        parent: Some(to_split),
        content: NodeType::Leaf(left),
    });
    let right = a.add(Node {
        parent: Some(to_split),
        content: NodeType::Leaf(right),
    });

    a.get_mut(to_split).content = NodeType::Pair { left, right };
}

#[allow(dead_code)]
fn display_tree(a: &Arena<Node>, root: NodeIdx) {
    fn _rec(a: &Arena<Node>, node_idx: NodeIdx) {
        let node = a.get(node_idx);
        match node.content {
            NodeType::Leaf(v) => print!("{}", v),
            NodeType::Pair { left, right } => {
                print!("[");
                _rec(a, left);
                print!(",");
                _rec(a, right);
                print!("]")
            }
        }
    }
    _rec(a, root);
    println!("")
}

fn tree_magnitude(a: &Arena<Node>, root: NodeIdx) -> u64 {
    match a.get(root).content {
        NodeType::Leaf(v) => v as u64,
        NodeType::Pair { left, right } => {
            3 * tree_magnitude(a, left) + 2 * tree_magnitude(a, right)
        }
    }
}

fn merge_trees(a: &mut Arena<Node>, left: NodeIdx, right: NodeIdx) -> NodeIdx {
    let tree = add_trees(a, left, right);
    loop {
        if let Some(to_explode) = find_lefmost_4_deep(a, tree) {
            explode_pair(a, to_explode);
            continue;
        }
        if let Some(to_split) = find_lefmost_bigger_10(a, tree) {
            split_leaf(a, to_split);
            continue;
        }
        break;
    }
    tree
}

fn step_1(a: &Arena<Node>, trees: &[NodeIdx]) {
    let (mut a, trees) = new_subtree_arena(a, trees);

    let mut tree = trees[0];
    for t in trees.iter().skip(1) {
        tree = merge_trees(&mut a, tree, *t);
    }
    let result = tree_magnitude(&a, tree);
    println!("Step 1 result: {}", result)
}

fn step_2(a: &Arena<Node>, trees: &[NodeIdx]) {
    let mut max_magn = 0;
    for (i, &first) in trees.iter().enumerate() {
        for &second in &trees[i + 1..] {
            let (mut sub_a, roots) = new_subtree_arena(&a, &[first, second]);
            let merged = merge_trees(&mut sub_a, roots[0], roots[1]);
            max_magn = max_magn.max(tree_magnitude(&sub_a, merged));

            let (mut sub_a, roots) = new_subtree_arena(&a, &[first, second]);
            let merged = merge_trees(&mut sub_a, roots[1], roots[0]);
            max_magn = max_magn.max(tree_magnitude(&sub_a, merged));
        }
    }
    println!("Step 2 result: {}", max_magn);
}

fn main() {
    let (arena, trees) = parse_input();
    step_1(&arena, &trees);
    step_2(&arena, &trees);
}
