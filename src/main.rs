mod grid;
mod tile;
mod coord;

extern crate revord;
extern crate ansi_term;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::rc::Rc;
use revord::RevOrd;

use grid::Grid;
use coord::Coord;

fn read_grid_file(path_str: &str) -> Result<Grid, &str> {
    // Create a path to the desired file
    let path = Path::new(path_str);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => {
            panic!(
                "couldn't open {}: {}",
                display, why.description()
            )
        },
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<i32>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!(
            "couldn't read {}: {}",
            display, why.description()
        ),
        Ok(_) => Ok(Grid::from(&s))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    cost: u32,
    heuristic: u32,
    coord: Coord,
    parent: Option<Rc<State>>,
}

impl State {
    fn estimated_cost(&self) -> u32 {
        self.cost + self.heuristic
    }

    fn backtrace(&self) -> Vec<Coord> {
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

fn search(grid: &Grid) -> Result<Vec<Coord>, &'static str> {
    // Create open and closed lists.
    let mut open: BinaryHeap<RevOrd<State>> = BinaryHeap::new();
    let mut closed: Vec<Coord> = Vec::new();

    // Add first state to open list.
    open.push(RevOrd(State {
        coord: grid.start,
        cost: 0,
        heuristic: 0,
        parent: None
    }));

    let mut steps = 0u32;

    // Keep grabbing the lowest cost state and expanding it.
    while let Some(RevOrd(state)) = open.pop() {

        //println!("{:?}: {:?} popping {:?}", steps, open.len(), state.coord);
        steps += 1;

        // Goal has been found.
        if state.coord == grid.goal {
            println!("found in {:?} steps", steps);
            return Ok(state.backtrace());
        }

        let neighbors = grid.expand(state.coord);
        let cost = state.cost + 1;
        let parent = Some(Rc::new(state));

        for coord in neighbors {
            if !closed.contains(&coord) {
                let state = State {
                    coord: coord,
                    cost: cost,
                    heuristic: Coord::distance(&coord, &grid.goal),
                    parent: parent.clone(),
                };
                open.push(RevOrd(state));
                closed.push(coord);
            }
        }
    }
    Err("goal not found")
}

fn main() {
    match read_grid_file("map2.txt") {
        Ok(grid) => {
            println!("{}", grid.to_string());
            if let Ok(solution) = search(&grid) {
                let mut grid = grid;
                grid.set_path(&solution);
                println!("solution of {} steps found! \n{}", solution.len(), grid.to_color_string());
            } else {
                println!("couldn't find goal");
            }
        },
        Err(err) => println!("Failed to read map: {:?}", err)
    }
}
