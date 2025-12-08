use std::collections::HashSet;

pub fn part1(data: &str) -> i64 {
    let points = data.split('\n').map(Point::from).collect::<Vec<_>>();
    let mut circuits = Circuits::from(&points);
    let points_by_distance = distances(&points);
    points_by_distance.iter().take(1000).for_each(|(p1, p2)| {
        circuits.merge(p1, p2);
    });
    circuits
        .sizes()
        .iter()
        .take(3)
        .fold(1 as usize, |acc, value| acc * value) as i64
}

pub fn part2(data: &str) -> i64 {
    let points = data.split('\n').map(Point::from).collect::<Vec<_>>();
    let mut circuits = Circuits::from(&points);
    let points_by_distance = distances(&points);
    let (p1, p2) = points_by_distance.iter().skip_while(|(p1, p2)| {
        circuits.merge(p1, p2);
        circuits.circuits.len() > 1
    }).next().unwrap();
    
    p1.x * p2.x
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let xyz = value
            .split(',')
            .map(|part| part.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        Self::new(xyz[0], xyz[1], xyz[2])
    }
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x: x, y: y, z: z }
    }

    fn dist_squared(&self, other: &Self) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx.pow(2) + dy.pow(2) + dz.pow(2)
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Circuit {
    points: HashSet<Point>,
}

impl From<&Point> for Circuit {
    fn from(value: &Point) -> Self {
        Self {
            points: HashSet::from_iter([value.clone()]),
        }
    }
}

impl Circuit {
    fn extend(&mut self, c: &mut Circuit) {
        self.points.extend(c.points.iter().cloned());
    }

    fn has(&self, p: &Point) -> bool {
        self.points.contains(p)
    }

    fn len(&self) -> usize {
        self.points.len()
    }
}

#[derive(Debug)]
struct Circuits {
    circuits: Vec<Circuit>,
}

impl From<&Vec<Point>> for Circuits {
    fn from(value: &Vec<Point>) -> Self {
        Self {
            circuits: value.into_iter().map(Circuit::from).collect(),
        }
    }
}

impl Circuits {
    fn pop(&mut self, p: &Point) -> Circuit {
        let (index, _) = self
            .circuits
            .iter()
            .enumerate()
            .filter(|(_i, c)| c.has(p))
            .last()
            .unwrap();
        self.circuits.swap_remove(index)
    }

    fn merge(&mut self, p1: &Point, p2: &Point) {
        let mut c1 = self.pop(p1);
        if !c1.has(p2) {
            let mut c2 = self.pop(p2);
            c1.extend(&mut c2);
        }
        self.circuits.push(c1);
    }

    fn sizes(&self) -> Vec<usize> {
        let mut sizes = self.circuits.iter().map(|c| c.len()).collect::<Vec<_>>();
        sizes.sort_by_key(|size| -(*size as i64));
        sizes
    }
}

fn distances(points: &Vec<Point>) -> Vec<(Point, Point)> {
    let mut points_and_distances = Vec::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let p1 = points[i].clone();
            let p2 = points[j].clone();
            points_and_distances.push((p1.dist_squared(&p2), p1, p2));
        }
    }
    points_and_distances.sort();
    points_and_distances
        .into_iter()
        .map(|(_dist, p1, p2)| (p1, p2))
        .collect()
}
