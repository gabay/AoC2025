pub fn part1(data: &str) -> usize {
    let mut parts = data.split("\n\n").collect::<Vec<_>>();
    let floors = parts
        .pop()
        .unwrap()
        .split('\n')
        .map(Floor::from)
        .collect::<Vec<_>>();
    let shapes = parts
        .iter()
        .map(|part| Shape::from(*part))
        .collect::<Vec<_>>();

    floors.iter().filter(|floor| floor.can_tile(&shapes)).count()
}

pub fn part2(_data: &str) -> usize {
    0
}

#[derive(Clone)]
struct Shape(Vec<Vec<bool>>);
// struct Shape(vec<vec<bool>>);

impl From<&str> for Shape {
    fn from(value: &str) -> Self {
        let (_id_str, shape_str) = value.split_once('\n').unwrap();
        let shape = shape_str
            .split('\n')
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();
        Self(shape)
    }
}

impl Shape {
    fn exact_area(&self) -> usize {
        self.0.iter().flat_map(|row| row.iter()).filter(|b| **b).count()
    }
    fn overestimated_area(&self) -> usize {
        self.width() * self.height()
    }
    fn width(&self) -> usize {
        self.0[0].len()
    }
    fn height(&self) -> usize {
        self.0.len()
    }
}

struct Floor {
    width: usize,
    height: usize,
    needed_shapes: Vec<usize>,
}

impl From<&str> for Floor {
    fn from(value: &str) -> Self {
        let (xy, shapes) = value.split_once(": ").unwrap();
        let (width, height) = xy.split_once('x').unwrap();
        Self {
            width: width.parse().unwrap(),
            height: height.parse().unwrap(),
            needed_shapes: shapes.split(' ').map(|i| str::parse(i).unwrap()).collect(),
        }
    }
}

impl Floor {
    fn can_tile(&self, shapes: &Vec<Shape>) -> bool {
        if self.is_trivially_possible(shapes) {
            true
        } else if self.is_trivially_impossible(shapes) {
            false
        } else {
            panic!("Cannot determine trivially if floor can tile shapes.");
        }
    }

    fn is_trivially_possible(&self, shapes: &Vec<Shape>) -> bool {
        shapes
            .iter()
            .zip(&self.needed_shapes)
            .map(|(shape, count)| shape.overestimated_area() * count)
            .sum::<usize>()
            <= self.width * self.height
    }

    fn is_trivially_impossible(&self, shapes: &Vec<Shape>) -> bool {
        shapes
            .iter()
            .zip(&self.needed_shapes)
            .map(|(shape, count)| shape.exact_area() * count)
            .sum::<usize>()
            > self.width * self.height
    }
}
