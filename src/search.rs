use std::collections::{ BinaryHeap, HashSet };
use std::cmp::Ordering;
use std::rc::Rc;
use std::hash::Hash;

pub trait Space {
    type T;
    fn start(&self) -> Self::T;
    fn is_goal(&self, state: &Self::T) -> bool;
    fn expand(&self, state: &Self::T) -> Vec<Self::T>;
    fn distance(&self, from: &Self::T, to: &Self::T) -> u32;
    fn heuristic(&self, from: &Self::T) -> u32;
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Node<T> {
    g_score: u32,
    h_score: u32,
    state: Rc<T>,
    parent: Option<Rc<Node<T>>>,
}

impl<T> Node<T> {
    fn from_start(state: T) -> Node<T> {
        Node {
            g_score: 0,
            h_score: 0,
            state: Rc::new(state),
            parent: None,
        }
    }

    fn estimated_cost(&self) -> u32 {
        self.g_score + self.h_score
    }

    fn backtrace(&self) -> Vec<Rc<T>> {
        let mut result = vec![self.state.clone()];
        let mut node = self;
        while let Some(ref parent) = node.parent {
            result.push(parent.state.clone());
            node = parent;
        }
        result.reverse();
        result
    }
}

impl<T: PartialEq> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.estimated_cost().cmp(&other.estimated_cost()).reverse())
    }
}

impl<T: Eq> Ord for Node<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.estimated_cost().cmp(&other.estimated_cost()).reverse()
    }
}

pub struct SearchResult<T> {
    pub path: Vec<Rc<T>>,
    pub cost: u32,
    pub expansion_count: u32,
}

pub fn search<S: Space<T = T>, T: Eq + Hash>(space: &S) -> Option<SearchResult<T>> {
    // Create open and closed lists.
    let mut open : BinaryHeap<Node<T>> = BinaryHeap::new();
    let mut closed : HashSet<Rc<T>> = HashSet::new();

    // Add first state to open list.
    let start = Node::from_start(space.start());
    open.push(start);

    // Keep grabbing the lowest cost state and expanding it.
    while let Some(node) = open.pop() {
        // Goal has been found.
        if space.is_goal(&node.state) {
            return Some(SearchResult {
                path: node.backtrace(),
                cost: node.g_score,
                expansion_count: closed.len() as u32,
            });
        }

        // Clone reference.
        let state = node.state.clone();
        let g_score = node.g_score;
        let parent : Rc<Node<T>> = Rc::new(node);

        for neighbor in space.expand(&state) {
            if !closed.contains(&neighbor) {
                let neighbor = Rc::new(neighbor);
                let node = Node {
                    state: neighbor.clone(),
                    g_score: g_score + space.distance(&state, &neighbor),
                    h_score: space.heuristic(&neighbor),
                    parent: Some(parent.clone()),
                };
                open.push(node);
            }
        }

        // Add state to closed list.
        closed.insert(state.clone());
    }
    None
}
