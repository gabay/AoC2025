use crate::util::Point;

use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::float::single::SingleFloatOverlay;
use i_overlay::i_shape::float::area::Area;

pub fn part1(data: &str) -> i64 {
    let points = data
        .split('\n')
        .map(|p| p.split_once(',').unwrap())
        .map(|(x, y)| Point::from(x.parse().unwrap(), y.parse().unwrap()))
        .collect::<Vec<_>>();

    let mut largest_area = 0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            largest_area = std::cmp::max(largest_area, rect_area(&points[i], &points[j]));
        }
    }
    largest_area
}

pub fn part2(data: &str) -> i64 {
    let points = data
        .split('\n')
        .map(|p| p.split_once(',').unwrap())
        .map(|(x, y)| [x.parse().unwrap(), y.parse().unwrap()])
        .collect::<Vec<_>>();
    let shape = [points.clone()];

    let mut largest_area = 0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let (x1, x2) = minmax(points[i][0], points[j][0]);
            let (y1, y2) = minmax(points[i][1], points[j][1]);
            let rect = [[x1, y1], [x2, y1], [x2, y2], [x1, y2]];
            let area = rect_area2(&points[i], &points[j]);
            if largest_area < area
                && rect
                    .overlay(&shape, OverlayRule::Difference, FillRule::EvenOdd)
                    .area()
                    == 0.0
            {
                largest_area = area;
            }
        }
    }
    largest_area
}

fn rect_area(p1: &Point, p2: &Point) -> i64 {
    let dx = (p1.x - p2.x).abs() as i64;
    let dy = (p1.y - p2.y).abs() as i64;
    (dx + 1) * (dy + 1)
}

fn rect_area2(p1: &[f64; 2], p2: &[f64; 2]) -> i64 {
    let dx = (p1[0] - p2[0]).abs() as i64;
    let dy = (p1[1] - p2[1]).abs() as i64;
    (dx + 1) * (dy + 1)
}

fn minmax(a: f64, b: f64) -> (f64, f64) {
    if a < b { (a, b) } else { (b, a) }
}
