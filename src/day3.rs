pub fn part1(data: &str) -> u64 {
    data.split('\n')
        .map(|line| get_max_2_digits(line.chars().map(|c| c.to_digit(10).unwrap()).collect()))
        .sum()
}

pub fn part2(data: &str) -> u64 {
    data.split('\n')
        .map(|line| get_max_n_digits(12, line.chars().map(|c| c.to_digit(10).unwrap()).collect()))
        .sum()
}

fn get_max_2_digits(digits: Vec<u32>) -> u64 {
    get_max_n_digits(2, digits)
    // let mut max = 0;
    // for j in 1..digits.len() {
    //     let local_max = digits.iter().take(j).max().unwrap() * 10 + digits[j] as;
    //     if max < local_max {
    //         max = local_max;
    //     }
    // }
    // max
}

fn get_max_n_digits(n: u32, digits: Vec<u32>) -> u64 {
    if n == 1 {
        digits.iter().max().unwrap().clone() as u64
    } else {
        let mut index = 0;
        for i in 1..digits.len() + 1 - n as usize {
            if digits[index] < digits[i] {
                index = i;
            }
        }
        digits[index] as u64 * 10u64.pow(n - 1)
            + get_max_n_digits(n - 1, digits.into_iter().skip(index + 1).collect())
    }
}
