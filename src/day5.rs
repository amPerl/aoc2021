use std::collections::HashMap;

use itertools::Itertools;

fn parse_input(input: &str) -> Vec<((usize, usize), (usize, usize))> {
    input
        .split_whitespace()
        // ["0,9", "->", "5,9", "8,0", "->", "0,8", ..]
        .filter(|&w| w != "->")
        // ["0,9", "5,9", "8,0", "0,8", ..]
        .flat_map(|coords| coords.split(','))
        // ["0", "9", "5", "9", "8", "0", "0", "8", ..]
        .map(|num| num.parse::<usize>().unwrap())
        // [0, 9, 5, 9, 8, 0, 0, 8, ..]
        .chunks(2)
        .into_iter()
        .map(|mut ch| (ch.next().unwrap(), ch.next().unwrap()))
        // [(0, 9), (5, 9), (8, 0), (0, 8), ..]
        .chunks(2)
        .into_iter()
        .map(|mut ch| (ch.next().unwrap(), ch.next().unwrap()))
        // [((0, 9), (5, 9)), ((8, 0), (0, 8)), ..]
        .collect()
}

fn solve(lines: Vec<((usize, usize), (usize, usize))>, axis_aligned: bool) -> usize {
    let mut map: HashMap<(usize, usize), usize> = HashMap::new();

    for ((x1, y1), (x2, y2)) in lines {
        let x_diff = x2 as isize - x1 as isize;
        let y_diff = y2 as isize - y1 as isize;

        let x_step = x_diff.signum();
        let y_step = y_diff.signum();

        if x_step != 0 && y_step != 0 && axis_aligned {
            continue;
        }

        let step_count = x_diff.abs().max(y_diff.abs()) + 1;

        for i in 0..step_count {
            let x = (x1 as isize + i * x_step) as usize;
            let y = (y1 as isize + i * y_step) as usize;
            *map.entry((x, y)).or_default() += 1;
        }
    }

    map.values().filter(|&&x| x > 1).count()
}

pub fn part1(input: &str) -> usize {
    solve(parse_input(input), true)
}

pub fn part2(input: &str) -> usize {
    solve(parse_input(input), false)
}
