extern crate revord;
extern crate ansi_term;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::ops::Add;
use std::rc::Rc;
use revord::RevOrd;
use ansi_term::Colour::Red;
use ansi_term::Colour::Green;
use ansi_term::Colour::Blue;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Open,
    Blocked,
    Start,
    Goal
}

impl Tile {
    fn from(c: char) -> Tile {
        match c {
            '#' => Tile::Blocked,
            's' => Tile::Start,
            'g' => Tile::Goal,
            _ => Tile::Open,
        }
    }

    fn to_char(&self) -> char {
        match self {
            &Tile::Open    => ' ',
            &Tile::Blocked => '#',
            &Tile::Start   => 's',
            &Tile::Goal    => 'g',
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(x: i32, y:i32) -> Coord {
        Coord { x: x, y: y }
    }

    fn distance(from: &Coord, to: &Coord) -> u32 {
        ((to.y - from.y).abs() + (to.y - from.y).abs()) as u32
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, other: Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
#[test]
fn coord_eq_test() {
    let coords = vec![Coord::new(1, 1)];
    assert!(coords.contains(&Coord::new(1, 1)));
}

#[derive(Debug)]
struct Grid {
    start: Coord,
    goal: Coord,
    width: i32,
    height: i32,
    tiles: Vec<Vec<Tile>>,
}

fn parse_integers(string: &str) -> Vec<i32> {
    string.split(' ').map(|s|
        s.parse().expect("Invalid dimension")
    ).collect()
}

impl Grid {
    fn from(string: &str) -> Grid {
        let mut lines = string.split('\n');
        if let Some(line) = lines.next() {
            let dimensions = parse_integers(line);
            let mut start: Option<Coord> = None;
            let mut goal: Option<Coord> = None;
            let mut tiles: Vec<Vec<Tile>> = Vec::new();

            for (y, line) in lines.enumerate() {
                tiles.push(line.chars().enumerate().map(|(x, symbol)| {
                    let tile = Tile::from(symbol);
                    match tile {
                        Tile::Start => {
                            start = Some(Coord { x: x as i32, y: y as i32 })
                        },
                        Tile::Goal => {
                            if let Some(prev) = goal {
                                panic!(
                                    "Found goal at {:?}, but goal was already found at {:?}",
                                    Coord::new(x as i32, y as i32), prev
                                );
                            }
                            goal = Some(Coord { x: x as i32, y: y as i32 })
                        },
                        _ => {}
                    }
                    tile
                }).collect())
            }

            Grid {
                start: start.expect("Start not found!"),
                goal: goal.expect("Goal not found!"),
                width: dimensions[0],
                height: dimensions[1],
                tiles: tiles
            }
        } else {
            panic!("Empty file");
        }
    }

    fn to_string(&self) -> String {
        self.tiles.iter().map(|row|
             row.iter()
                 .map(|tile| tile.to_char())
                 .collect::<String>()
         ).collect::<Vec<_>>().join("\n")
    }

    fn tile_at(&self, coord: &Coord) -> Option<Tile> {
        if self.in_range(coord) {
            Some(self.tiles[coord.y as usize][coord.x as usize])
        } else {
            None
        }
    }

    fn in_range(&self, coord: &Coord) -> bool {
        coord.x >= 0 &&
            coord.x < self.width &&
            coord.y >= 0 &&
            coord.y < self.height
    }

    fn expand(&self, coord: Coord) -> Vec<Coord> {
        let offsets = [
            // Up
            Coord::new(0, -1),
            // Left
            Coord::new(-1, 0),
            // Right
            Coord::new(1, 0),
            // Down
            Coord::new(0, 1),
        ];

        offsets.iter().fold(Vec::new(), |mut neighbors, offset| {
            let neighbor_coord = coord + *offset;
            match self.tile_at(&neighbor_coord) {
                None | Some(Tile::Blocked) => {},
                _ => neighbors.push(neighbor_coord),
            }
            neighbors
        })
    }
}

#[test]
fn expand_test() {
    let grid = Grid {
        tiles: vec![
            vec![Tile::Open, Tile::Open, Tile::Open],
            vec![Tile::Open, Tile::Open, Tile::Blocked],
            vec![Tile::Open, Tile::Open, Tile::Open],
        ],
        start: Coord::new(0, 0),
        goal: Coord::new(0, 0),
        width: 3,
        height: 3,
    };

    assert_eq!(
        grid.expand(Coord::new(1, 1)),
        [Coord::new(1, 0), Coord::new(0, 1), Coord::new(1, 2)]
    );

    assert_eq!(
        grid.expand(Coord::new(2, 2)),
        [Coord::new(1, 2)]
    );
}

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

fn backtrace(state: &State) -> Vec<Coord> {
    let mut result = vec![state.coord];
    let mut state = state;
    while let Some(ref next) = state.parent {
        result.push(next.coord);
        state = next;
    }
    result.reverse();
    result
}

/*
struct Backtrace {
    curr: Option<Rc<State>>,
}

impl Iterator for Backtrace {
    type Item = Rc<State>;
    fn next(&mut self) -> Option<Rc<State>> {
        let result = self.curr.clone();
        self.curr = match self.curr {
            None => None,
            Some(ref state) => state.parent.clone(),
        };
        result
    }
}
*/

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
            return Ok(backtrace(&state));
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

fn solution_to_string(grid: &Grid, path: Vec<Coord>) -> String {
    grid.tiles.iter().enumerate().map(|(y, row)|
         row.iter()
             .enumerate()
             .map(|(x, tile)|
                if path.contains(&Coord::new(x as i32, y as i32)) {
                    match tile {
                        &Tile::Start => Blue.paint("s"),
                        &Tile::Goal => Green.paint("✓"),
                        _ => Red.paint("•")
                    }.to_string()
                } else {
                    tile.to_char().to_string()
                }
             )
             .collect::<String>()
     ).collect::<Vec<_>>().join("\n")
}

fn main() {
    match read_grid_file("maze100.txt") {
        Ok(grid) => {
            println!("{}", grid.to_string());
            if let Ok(solution) = search(&grid) {
                println!("solution of {} steps found! \n{}", solution.len(), solution_to_string(&grid, solution));
            } else {
                println!("couldn't find goal");
            }
        },
        Err(err) => println!("Failed to read map: {:?}", err)
    }
}
