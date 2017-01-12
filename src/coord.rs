use std::ops::Add;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn new(x: i32, y:i32) -> Coord {
        Coord { x: x, y: y }
    }

    pub fn distance(from: &Coord, to: &Coord) -> u32 {
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
