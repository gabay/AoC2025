use std::collections::HashSet;

use crate::util::Point;

pub fn part1(data: &str) -> usize {
    Matrix::from(data).count_splits()
}

pub fn part2(data: &str) -> usize {
    Matrix::from(data).count_timelines()
}

struct Matrix {
    elements: Vec<Vec<char>>,
}

impl From<&str> for Matrix {
    fn from(value: &str) -> Self {
        Self {
            elements: value.split('\n').map(|row| row.chars().collect()).collect(),
        }
    }
}

impl Matrix {
    fn at(&self, p: &Point) -> Option<char> {
        if p.y >= 0
            && p.y < self.elements.len() as i32
            && p.x >= 0
            && p.x < self.elements[p.y as usize].len() as i32
        {
            Some(self.elements[p.y as usize][p.x as usize])
        } else {
            None
        }
    }

    fn start(&self) -> Point {
        for y in 0..self.elements.len() {
            for x in 0..self.elements[y].len() {
                if self.elements[y][x] == 'S' {
                    return Point::from(x, y);
                }
            }
        }
        panic!("no start");
    }

    fn count_splits(&self) -> usize {
        let mut points = vec![self.start()];
        let mut splits = HashSet::new();
        while let Some(point) = points.pop() {
            let next_point = point.down();
            match self.at(&next_point) {
                Some('.') => points.push(next_point),
                Some('^') => {
                    if splits.insert(next_point.clone()) {
                        points.append(&mut vec![next_point.left(), next_point.right()])
                    }
                }
                None => {}
                _ => panic!("???"),
            }
        }
        splits.len()
    }

    fn count_timelines(&self) -> usize {
        let mut weights: Vec<Vec<usize>> = Vec::from_iter(
            self.elements
                .iter()
                .map(|line| line.iter().map(|_| 0usize).collect()),
        );
        let start = self.start();
        weights[start.y as usize][start.x as usize] = 1;
        let mut count = 0;
        for y in 0..self.elements.len() {
            for x in 0..self.elements[y].len() {
                let weight = weights[y][x];
                if weight == 0 {
                    continue;
                }

                match self.at(&Point::from(x, y + 1)) {
                    Some('.') => weights[y + 1][x] += weight,
                    Some('^') => {
                        weights[y + 1][x - 1] += weight;
                        weights[y + 1][x + 1] += weight;
                    }
                    None => count += weight,
                    _ => panic!("???"),
                }
            }
        }
        count
    }
}
