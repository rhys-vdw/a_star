use coord::Coord;
use tile::Tile;

#[derive(Debug)]
pub struct Grid {
    pub start: Coord,
    pub goal: Coord,
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Vec<Tile>>,
}

fn parse_integers(string: &str) -> Vec<i32> {
    string.split(' ').map(|s|
        s.parse().expect("Invalid dimension")
    ).collect()
}

impl Grid {
    pub fn from(string: &str) -> Grid {
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
                                    "Goal specified at {:?}, but goal was already specified at {:?}",
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
                start: start.expect("Start not specified!"),
                goal: goal.expect("Goal not specified!"),
                width: dimensions[0],
                height: dimensions[1],
                tiles: tiles
            }
        } else {
            panic!("Empty file");
        }
    }

    pub fn set_path(&mut self, path: &[Coord]) {
        for coord in path {
            let tile = self.tiles[coord.y as usize][coord.x as usize];
            if tile != Tile::Start && tile != Tile::Goal {
                self.tiles[coord.y as usize][coord.x as usize] = Tile::Path;
            }
        }
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        self.tiles.iter().map(|row|
             row.iter()
                 .map(|tile| tile.to_char())
                 .collect::<String>()
         ).collect::<Vec<_>>().join("\n")
    }

    pub fn to_color_string(&self) -> String {
        self.tiles.iter().map(|row|
             row.iter()
                 .map(|tile| tile.to_color_string())
                 .collect::<String>()
         ).collect::<Vec<_>>().join("\n")
    }

    pub fn tile_at(&self, coord: &Coord) -> Option<Tile> {
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

    pub fn expand(&self, coord: Coord) -> Vec<Coord> {
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

