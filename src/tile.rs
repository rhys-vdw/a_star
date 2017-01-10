use ansi_term::Colour::Red;
use ansi_term::Colour::Green;
use ansi_term::Colour::Blue;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tile {
    Open,
    Blocked,
    Start,
    Goal,
    Path
}

impl Tile {
    pub fn from(c: char) -> Tile {
        match c {
            '#' => Tile::Blocked,
            's' => Tile::Start,
            'g' => Tile::Goal,
            _ => Tile::Open,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            &Tile::Open    => ' ',
            &Tile::Blocked => '#',
            &Tile::Start   => 's',
            &Tile::Goal    => 'g',
            &Tile::Path    => '•',
        }
    }

    pub fn to_color_string(&self) -> String {
        match self {
            &Tile::Open    => " ".to_string(),
            &Tile::Blocked => Blue.paint("#").to_string(),
            &Tile::Start   => Red.paint("s").to_string(),
            &Tile::Goal    => Green.paint("✓").to_string(),
            &Tile::Path    => Red.paint("•").to_string(),
        }
    }
}
