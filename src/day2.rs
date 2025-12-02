struct Range {
    a: u64,
    b: u64,
}

impl Range {
    fn from_string(s: &str) -> Self {
        let (a, b) = s.split_once('-').unwrap();
        Range {
            a: a.parse::<u64>().unwrap(),
            b: b.parse::<u64>().unwrap(),
        }
    }

    fn sum_invalids1(&self) -> u64 {
        let mut sum = 0;
        for i in self.a..=self.b {
            let digits = count_digits(i);
            if digits % 2 == 0 {
                if is_pattern_with_length(i, digits / 2) {
                    sum += i;
                }
            }
        }
        sum
    }

    fn sum_invalids2(&self) -> u64 {
        let mut sum = 0;
        for i in self.a..=self.b {
            let digits = count_digits(i);
            for j in 1..=(digits / 2) {
                if is_pattern_with_length(i, j) {
                    sum += i;
                    break;
                }
            }
        }
        sum
    }
}

fn count_digits(n: u64) -> u32 {
    let mut digits: u32 = 1;
    while 10u64.pow(digits) <= n {
        digits += 1;
    }
    digits
}

fn is_pattern_with_length(mut n: u64, length: u32) -> bool {
    if n == 0 {
        return false;
    }
    let digits = count_digits(n);
    if digits % length != 0 {
        return false;
    }
    let pattern = n / 10u64.pow(digits - length);
    let exponent = 10u64.pow(length);
    while n > 0 {
        if n % exponent != pattern {
            return false;
        }
        n = n / exponent;
    }
    return true;
}


pub fn part1(data: &str) -> u64 {
    let ranges = data.split(',').map(Range::from_string);
    ranges.map(|r| r.sum_invalids1()).sum()
}

pub fn part2(data: &str) -> u64 {
    let ranges = data.split(',').map(Range::from_string);
    ranges.map(|r| r.sum_invalids2()).sum()
}
