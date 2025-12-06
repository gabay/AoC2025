pub fn part1(data: &str) -> i64 {
    let elements = data
        .split('\n')
        .map(|line| {
            line.split(' ')
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let transposed = transpose(elements);
    transposed.into_iter().map(part_1_calculate).sum()
}

pub fn part2(data: &str) -> i64 {
    let mat = Matrix::from(data);
    let mut calc = Part2Calculator::new();
    for col in mat.columns() {
        if col.iter().all(|c| c.is_whitespace()) {
            calc.reset();
        } else {
            calc.digest(col);
        }
    }
    calc.reset();
    calc.result
}

fn transpose<T: Copy>(m: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    for i in 0..m[0].len() {
        result.push(m.iter().map(|line| line[i]).collect());
    }
    result
}

fn part_1_calculate(mut items: Vec<&str>) -> i64 {
    match items.pop().unwrap() {
        "+" => items.iter().map(|i| i.parse::<i64>().unwrap()).sum(),
        "*" => items
            .iter()
            .map(|i| i.parse::<i64>().unwrap())
            .fold(1, |acc, i| acc * i),
        _ => panic!(),
    }
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
    fn columns(&self) -> Vec<Vec<char>> {
        Vec::from_iter(
            (0..self.elements[0].len()).map(|i| self.elements.iter().map(|row| row[i]).collect()),
        )
    }
}

struct Part2Calculator {
    items: Vec<i64>,
    operator: Option<char>,
    result: i64,
}

impl Part2Calculator {
    fn new() -> Self {
        Self {
            items: Vec::new(),
            operator: None,
            result: 0,
        }
    }

    fn digest(&mut self, element: Vec<char>) {
        if element.contains(&'+') {
            self.operator = Some('+');
        }
        if element.contains(&'*') {
            self.operator = Some('*');
        }
        self.items.push(
            element
                .into_iter()
                .filter(char::is_ascii_digit)
                .collect::<String>()
                .parse()
                .unwrap(),
        );
    }

    fn calculate(&self) -> i64 {
        match self.operator {
            Some('+') => self.items.iter().sum(),
            Some('*') => self.items.iter().fold(1, |acc, item| acc * item),
            _ => panic!(),
        }
    }

    fn reset(&mut self) {
        self.result += self.calculate();
        self.items.clear();
        self.operator = None;
    }
}
