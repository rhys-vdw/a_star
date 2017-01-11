use std::rc::Rc;
use coord::Coord;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub struct State {
    pub cost: u32,
    pub heuristic: u32,
    pub coord: Coord,
    pub parent: Option<Rc<State>>,
}

impl State {
    fn estimated_cost(&self) -> u32 {
        self.cost + self.heuristic
    }

    pub fn backtrace(&self) -> Vec<Coord> {
        let mut result = vec![self.coord];
        let mut state = self;
        while let Some(ref next) = state.parent {
            result.push(next.coord);
            state = next;
        }
        result.reverse();
        result
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.estimated_cost().cmp(&other.estimated_cost())
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}
