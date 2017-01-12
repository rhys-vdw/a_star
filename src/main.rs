mod grid;
mod tile;
mod coord;
mod search;

extern crate revord;
extern crate ansi_term;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;
use search::SearchResult;

use grid::Grid;

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

fn run_search(grid: &mut Grid) {
    println!("Searching...");
    match search::search(&*grid) {
        Some(SearchResult { path, cost, expansion_count }) => {
            let path : Vec<_> = path.iter().map(|v| {
                *v.clone()
            }).collect();
            grid.set_path(&path);
            println!(
                "...Solution found!\n\
                {}\n\
                cost: {}\n\
                expansions: {}\n",
                grid.to_color_string(),
                cost,
                expansion_count
            );
        },
        None => println!("Goal not found")
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
