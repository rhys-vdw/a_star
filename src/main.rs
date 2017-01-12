mod grid;
mod tile;
mod coord;
mod state;

extern crate revord;
extern crate ansi_term;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use revord::RevOrd;
use std::rc::Rc;
use std::env;

use grid::Grid;
use coord::Coord;
use state::State;

fn read_grid_file(path_str: &str) -> Result<Grid, String> {
    // Create a path to the desired file
    let path = Path::new(path_str);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => return Err(format!(
            "Couldn't open `{}`: {}", display, why.description()
        )),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<i32>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => return Err(format!(
            "Couldn't read `{}`: {}", display, why.description()
        )),
        Ok(_) => Ok(Grid::from(&s))
    }
}

fn search(grid: &Grid) -> Result<Vec<Coord>, &'static str> {
    // Create open and closed lists.
    let mut open: BinaryHeap<RevOrd<State>> = BinaryHeap::new();
    let mut closed = HashSet::new();

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
                closed.insert(coord);
            }
        }
    }
    Err("goal not found")
}

fn run_search(grid: &mut Grid) {
    println!("Searching...");
    match search(&grid) {
        Ok(solution) => {
            grid.set_path(&solution);
            println!(
                "...Solution of {} steps found! \n{}",
                solution.len(), grid.to_color_string()
            );
        },
        Err(why) => println!("...{}", why)
    }
}

fn main() {
    if let Some(file_name) = env::args().nth(1) {
        match read_grid_file(&file_name) {
            Ok(mut grid) => {
                println!("Successfully loaded `{}`", file_name);
                run_search(&mut grid)
            },
            Err(err) => println!("{}", err)
        }
    } else {
        println!("Usage: map <filename>");
    }
}
