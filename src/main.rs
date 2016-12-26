use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Clone, Copy, Debug)]
enum Cell {
    Open,
    Blocked,
    Start,
    Goal
}

impl Cell {
    fn from(c: char) -> Cell {
        match c {
            '.' => Cell::Open,
            '#' => Cell::Blocked,
            's' => Cell::Start,
            'g' => Cell::Goal,
            _ => panic!("Unrecognized node: {}", c)
        }
    }
}

struct Node {
    x: i32,
    y: i32,
    cell: Cell,
}

#[derive(Debug)]
struct Grid {
    width: i32,
    height: i32,
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    fn from(string: &str) -> Grid {
        let mut lines = string.split('\n');
        if let Some(line) = lines.next() {
            let dimensions: Vec<_> = line.split(' ').map(|s|
                s.parse::<i32>().expect("Invalid dimension")
            ).collect();

            let cells: Vec<Vec<_>> = lines
                .take(dimensions[1] as usize)
                .map(|line|
                  line.chars().map(Cell::from).collect()
                ).collect();

            Grid {
                width: dimensions[0],
                height: dimensions[1],
                cells: cells
            }
        } else {
            panic!("Empty file");
        }
    }

    fn expand(&self, node: Node) -> Vec<Node> {
        let mut result = Vec::new();
        for y in (node.y - 1)..(node.y + 2) {
            if y >= 0 || y < self.height {
                for x in (node.x - 1)..(node.x + 2) {
                    if !(x == node.x && y == node.y) && x < 0 || x >= self.width {
                        let cell = self.cells[y as usize][x as usize];
                        match cell {
                          Cell::Blocked => {},
                          _ => result.push(Node { x: x, y: y, cell: cell })
                        }
                    }
                }
            }
        }
        result
    }
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

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!(
            "couldn't read {}: {}",
            display, why.description()
        ),
        Ok(_) => Ok(Grid::from(&s))
    }
}

fn main() {
    let grid = read_grid_file("map.txt");
    println!("{:?}", grid.unwrap());
}
