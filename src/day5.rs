use std::cmp;

pub fn part1(data: &str) -> usize {
    let (ranges_str, items_str) = data.split_once("\n\n").unwrap();
    let ranges: Vec<Range> = ranges_str.split('\n').map(Range::from).collect();
    items_str
        .split('\n')
        .filter(|item| is_in_ranges(*item, &ranges))
        .count()
}

pub fn part2(data: &str) -> usize {
    let (ranges_str, _) = data.split_once("\n\n").unwrap();
    let ranges: Vec<Range> = ranges_str.split('\n').map(Range::from).collect();
    let multirange = MultiRange::from(ranges);
    multirange.len()
}

fn is_in_ranges(item: &str, ranges: &Vec<Range>) -> bool {
    let item_i64 = item.parse::<i64>().unwrap();
    ranges.iter().any(|range| range.contains(item_i64))
}

#[derive(PartialEq, Eq)]
struct Range {
    a: i64,
    b: i64,
}

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let (a, b) = value.split_once('-').unwrap();
        Self {
            a: a.parse().unwrap(),
            b: b.parse().unwrap(),
        }
    }
}

impl Range {
    fn contains(&self, item: i64) -> bool {
        item >= self.a && item <= self.b
    }

    fn len(&self) -> usize {
        (self.b - self.a + 1) as usize
    }

    fn union_if_intersects(&self, other: &Self) -> Option<Self> {
        if self.a <= other.b && self.b >= other.a {
            Some(Self {
                a: cmp::min(self.a, other.a),
                b: cmp::max(self.b, other.b),
            })
        } else {
            None
        }
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match (self.a.cmp(&other.a), self.b.cmp(&other.b)) {
            (cmp::Ordering::Less, _) => cmp::Ordering::Less,
            (cmp::Ordering::Greater, _) => cmp::Ordering::Greater,
            (cmp::Ordering::Equal, b_ordering) => b_ordering,
        }
    }
}

struct MultiRange {
    ranges: Vec<Range>,
}

impl MultiRange {
    fn new() -> Self {
        Self { ranges: Vec::new() }
    }

    fn add(&mut self, range: Range) {
        let Some(last_range) = self.ranges.pop() else {
            self.ranges.push(range);
            return;
        };
        let Some(intersection) = last_range.union_if_intersects(&range) else {
            self.ranges.append(&mut vec![last_range, range]);
            return;
        };
        self.ranges.push(intersection);
    }

    fn len(self) -> usize {
        self.ranges.iter().map(Range::len).sum()
    }
}

impl From<Vec<Range>> for MultiRange {
    fn from(mut value: Vec<Range>) -> Self {
        let mut multirange = MultiRange::new();
        value.sort();
        value.into_iter().for_each(|range| multirange.add(range));
        multirange
    }
}