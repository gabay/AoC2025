

pub fn part1(data: &str) -> i32 {
    let mut dial = 50;
    let mut count = 0;
    data.split('\n').for_each(|line| {
        if line.starts_with('L') {
            let n = line.trim_matches('L').parse::<i32>().unwrap();
            dial = (dial - n) % 100;
        }
        if line.starts_with('R') {
            let n = line.trim_matches('R').parse::<i32>().unwrap();
            dial = (dial + n) % 100;
        }
        if dial == 0 {
            count += 1;
        }
    });
    count
}

pub fn part2(data: &str) -> i32 {
    let mut dial = 50;
    let mut count = 0;
    data.split('\n').for_each(|line| {
        if line.starts_with('L') {
            let n = line.trim_matches('L').parse::<i32>().unwrap();
            for _ in 0..n {
                dial = (dial - 1) % 100;
                if dial == 0 {
                    count += 1;
                }
            }
        }
        if line.starts_with('R') {
            let n: i32 = line.trim_matches('R').parse::<i32>().unwrap();
            for _ in 0..n {
                dial = (dial + 1) % 100;
                if dial == 0 {
                    count += 1;
                }
            }
        }
    });
    count
}
