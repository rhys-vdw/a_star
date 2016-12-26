use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
enum Node {
    Open,
    Blocked,
    Start,
    Goal
}

impl Node {
    fn from(c: char) -> Node {
        match c {
            '.' => Node::Open,
            '#' => Node::Blocked,
            's' => Node::Start,
            'g' => Node::Goal,
            _ => panic!("Unrecognized node: {}", c)
        }
    }
}

#[derive(Debug)]
struct Grid {
    width: u32,
    height: u32,
    nodes: Vec<Vec<Node>>,
}

impl Grid {
    fn from(string: &str) -> Grid {
        let mut lines = string.split('\n');
        if let Some(line) = lines.next() {
            let dimensions: Vec<_> = line.split(' ').map(|s|
                s.parse::<u32>().expect("Invalid dimension")
            ).collect();

            let nodes: Vec<Vec<_>> = lines
                .take(dimensions[1] as usize)
                .map(|line|
                  line.chars().map(Node::from).collect()
                ).collect();

            Grid {
                width: dimensions[0],
                height: dimensions[1],
                nodes: nodes
            }
        } else {
            panic!("Empty file");
        }
    }
}


fn main() {
    // Create a path to the desired file
    let path = Path::new("map.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!(
            "couldn't open {}: {}",
            display, why.description()
        ),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!(
            "couldn't read {}: {}",
            display, why.description()
        ),
        Ok(_) => {
            print!("{} contains:\n{}", display, s);
            let map = Grid::from(&s);
            println!("{:?}", map);
        }
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed
}
