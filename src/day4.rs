pub fn part1(data: &str) -> usize {
    let grid = Grid::from(data);

    let mut count = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            let p = Point::from(x, y);
            if grid.at(&p) == '@'
                && grid
                    .neighbors(&p)
                    .iter()
                    .filter(|p| grid.at(p) == '@')
                    .count()
                    < 4
            {
                count += 1;
            }
        }
    }
    count
}

pub fn part2(data: &str) -> usize {
    let mut grid = Grid::from(data);

    let mut count = 0;
    loop {
        let mut points_to_remove = Vec::new();
        for y in 0..grid.height {
            for x in 0..grid.width {
                let p = Point::from(x, y);
                if grid.at(&p) == '@'
                    && grid
                        .neighbors(&p)
                        .iter()
                        .filter(|p| grid.at(p) == '@')
                        .count()
                        < 4
                {
                    points_to_remove.push(p);
                }
            }
        }
        if points_to_remove.len() == 0 {
            break;
        }
        count += points_to_remove.len();
        points_to_remove.iter().for_each(|p| grid.set(p, '.'));
    }
    count
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn from(x: usize, y: usize) -> Self {
        Point {
            x: x as i32,
            y: y as i32,
        }
    }
    fn up(&self) -> Self {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn down(&self) -> Self {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn left(&self) -> Self {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn right(&self) -> Self {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }
}

struct Grid {
    cells: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl Grid {
    fn at(&self, p: &Point) -> char {
        self.cells[p.y as usize][p.x as usize]
    }

    fn set(&mut self, p: &Point, v: char) {
        self.cells[p.y as usize][p.x as usize] = v;
    }

    fn contains(&self, p: &Point) -> bool {
        p.y >= 0 && p.y < self.height as i32 && p.x >= 0 && p.x < self.width as i32
    }

    fn neighbors(&self, p: &Point) -> Vec<Point> {
        vec![
            p.up().left(),
            p.up(),
            p.up().right(),
            p.left(),
            p.right(),
            p.down().left(),
            p.down(),
            p.down().right(),
        ]
        .into_iter()
        .filter(|p| self.contains(p))
        .collect()
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let cells: Vec<Vec<char>> = value
            .split('\n')
            .map(|line| line.chars().collect())
            .collect();
        let height = cells[0].len();
        let width = cells.len();
        Grid {
            cells,
            height,
            width,
        }
    }
}
