use std::collections::{HashMap, HashSet, VecDeque};

pub fn part1(data: &str) -> i64 {
    let connections = Connections::from(data);
    connections.paths("you", "out")
}

pub fn part2(data: &str) -> i64 {
    let connections = Connections::from(data);

    let mut result = 0;
    if connections.distance("dac", "fft").is_some() {
        result += connections.paths("svr", "dac")
            * connections.paths("dac", "fft")
            * connections.paths("fft", "out");
    }
    if connections.distance("fft", "dac").is_some() {
        result += connections.paths("svr", "fft")
            * connections.paths("fft", "dac")
            * connections.paths("dac", "out");
    }
    result
}

struct Connection {
    name: String,
    outputs: Vec<String>,
}

impl From<&str> for Connection {
    fn from(value: &str) -> Self {
        let (name, outputs) = value.split_once(": ").unwrap();
        Self {
            name: name.to_string(),
            outputs: outputs.split(' ').map(str::to_string).collect(),
        }
    }
}

struct Connections {
    connections: HashMap<String, Connection>,
}

impl From<&str> for Connections {
    fn from(value: &str) -> Self {
        Self {
            connections: value
                .split('\n')
                .map(Connection::from)
                .map(|c| (c.name.to_string(), c))
                .collect(),
        }
    }
}

impl Connections {
    fn distance(&self, start: &str, end: &str) -> Option<i64> {
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((start, 0));
        while let Some((cell, depth)) = queue.pop_front() {
            if cell == end {
                return Some(depth);
            }
            if seen.insert(cell)
                && let Some(connection) = self.connections.get(cell)
            {
                connection
                    .outputs
                    .iter()
                    .for_each(|conn| queue.push_back((conn, depth + 1)));
            }
        }
        None
    }

    fn paths(&self, start: &str, end: &str) -> i64 {
        if let Some(distance) = self.distance(start, end) {
            self._paths(start, end, distance + 3)
        } else {
            0
        }
    }

    fn _paths(&self, start: &str, end: &str, depth: i64) -> i64 {
        if start == end {
            1
        } else if depth == 0 {
            0
        } else if let Some(cell) = self.connections.get(start) {
            cell.outputs
                .iter()
                .map(|output| self._paths(output, end, depth - 1))
                .sum()
        } else {
            0
        }
    }
}
