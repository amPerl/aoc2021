use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .tuple_windows()
        .map(|(a, b)| if b > a { 1 } else { 0 })
        .sum()
}

pub fn part2(input: &str) -> usize {
    input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .map(|(a, b)| if b > a { 1 } else { 0 })
        .sum()
}
