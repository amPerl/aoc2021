use std::{fmt::Debug, str::FromStr};

fn parse_input<T>(input: &str) -> Vec<T>
where
    T: FromStr,
    T::Err: Debug,
{
    input
        .trim()
        .split(',')
        .map(|x| x.parse::<T>().unwrap())
        .collect()
}

pub fn part1(input: &str) -> usize {
    let mut subs = parse_input::<isize>(input);
    subs.sort_unstable();

    let median = subs[subs.len() / 2];

    fn cost(a: isize, b: isize) -> isize {
        (a - b).abs()
    }

    subs.iter().map(|&pos| cost(pos, median)).sum::<isize>() as _
}

pub fn part2(input: &str) -> usize {
    let subs = parse_input(input);

    let pos_sum = subs.iter().sum::<usize>();
    let pos_avg = pos_sum / subs.len();

    fn sub_cost(a: usize, b: usize) -> usize {
        let distance = (a as isize - b as isize).abs() as usize;
        (distance.pow(2) + distance) / 2
    }

    fn fleet_cost(positions: &[usize], target_pos: usize) -> usize {
        positions.iter().map(|&pos| sub_cost(pos, target_pos)).sum()
    }

    let cost_a = fleet_cost(&subs, pos_avg);
    let cost_b = fleet_cost(&subs, pos_avg + 1);

    cost_a.min(cost_b)
}
