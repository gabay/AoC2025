use std::{fs, path::Path};

pub fn read(path: &str) -> Option<String> {
    Some(
        fs::read_to_string(Path::new(path))
            .ok()?
            .trim_matches('\n')
            .to_string(),
    )
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn from(x: usize, y: usize) -> Self {
        Point {
            x: x as i32,
            y: y as i32,
        }
    }
    pub fn up(&self) -> Self {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }
    pub fn down(&self) -> Self {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }
    pub fn left(&self) -> Self {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }
    pub fn right(&self) -> Self {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }
}