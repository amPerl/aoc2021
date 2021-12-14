use std::collections::HashMap;

use itertools::Itertools;

fn parse_input(input: &str) -> (String, HashMap<(char, char), char>) {
    let mut input = input.split_whitespace();

    let start = input.next().unwrap();
    let rules = input
        .chunks(3)
        .into_iter()
        .map(|mut line| {
            let mut from_chars = line.next().unwrap().chars();
            (
                (from_chars.next().unwrap(), from_chars.next().unwrap()),
                line.nth(1).unwrap().chars().next().unwrap(),
            )
        })
        .collect();

    (start.into(), rules)
}

fn solve(input: &str, iterations: usize) -> usize {
    let (start, rules) = parse_input(input);

    let mut pair_counts: HashMap<(char, char), usize> = HashMap::new();

    for (first, second) in start.chars().tuple_windows() {
        *pair_counts.entry((first, second)).or_default() += 1;
    }

    for _ in 0..iterations {
        let mut new_pair_counts: HashMap<(char, char), usize> = HashMap::new();
        for (pair, count) in pair_counts {
            if let Some(rule_insert) = rules.get(&pair) {
                *new_pair_counts.entry((pair.0, *rule_insert)).or_default() += count;
                *new_pair_counts.entry((*rule_insert, pair.1)).or_default() += count;
            } else {
                *new_pair_counts.entry(pair).or_default() += count;
            }
        }
        pair_counts = new_pair_counts;
    }

    let mut char_counts: HashMap<char, usize> = HashMap::new();

    for ((first, second), count) in pair_counts {
        *char_counts.entry(first).or_default() += count;
        *char_counts.entry(second).or_default() += count;
    }

    let mut counts_asc = char_counts
        .into_iter()
        .sorted_by(|a, b| Ord::cmp(&a.1, &b.1));

    let first = counts_asc.next().unwrap().1;
    let last = counts_asc.last().unwrap().1;

    let first = (first as f64 / 2.0).ceil();
    let last = (last as f64 / 2.0).ceil();

    (last - first) as usize
}

pub fn part1(input: &str) -> usize {
    solve(input, 10)
}

pub fn part2(input: &str) -> usize {
    solve(input, 40)
}
