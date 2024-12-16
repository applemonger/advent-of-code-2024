use std::{collections::HashMap, fmt::Display, ops::{Add, Sub}};

pub fn xy(x: i32, y: i32) -> XY {
    XY::new(x, y)
}

#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct XY {
    pub x: i32,
    pub y: i32,
}

impl XY {
    pub fn new(x: i32, y: i32) -> XY {
        XY { x, y }
    }

    pub fn neighbors(&self) -> [XY; 4] {
        [
            XY::new(self.x + 1, self.y), 
            XY::new(self.x - 1, self.y),
            XY::new(self.x, self.y + 1),
            XY::new(self.x, self.y - 1)
        ]
    }
}

pub fn cardinals() -> [XY; 4] {
    [XY::new(1, 0), XY::new(-1, 0), XY::new(0, 1), XY::new(0, -1)]
}

impl Add for XY {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        XY {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for XY {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        XY {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl From<(usize, usize)> for XY {
    fn from(value: (usize, usize)) -> Self {
        XY {
            x: value.0 as i32,
            y: value.1 as i32,
        }
    }
}

#[derive(Default)]
pub struct Grid {
    pub data: HashMap<XY, char>,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let data: HashMap<XY, char> = value
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| (XY::from((x, y)), c))
            })
            .collect();
        Grid { data }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let y_max = self.data.keys().map(|k| k.y).max().unwrap();
        let x_max = self.data.keys().map(|k| k.x).max().unwrap();
        let mut result = String::new();
        for y in 0..=y_max {
            let mut row = String::new();
            for x in 0..=x_max {
                let c = *self.data.get(&xy(x, y)).unwrap_or(&' ');
                row.push(c);
            }
            result += &row;
            result.push('\n');
        }
        write!(f, "{}", result)
    }
}

pub fn read_grid(input: &str) -> HashMap<XY, char> {
    let grid = Grid::from(input);
    grid.data.clone()
}

pub fn print_grid(grid: &HashMap<XY, char>) {
    let mut temp_grid = Grid::default();
    temp_grid.data = grid.clone();
    println!("{}", temp_grid);
}