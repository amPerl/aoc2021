use std::ops::RangeInclusive;

fn parse_input(input: &str) -> (RangeInclusive<isize>, RangeInclusive<isize>) {
    let ranges_str = input.split(':').nth(1).unwrap();
    let mut ranges_iter = ranges_str.split(',').map(|range_str| {
        // y=-10..-5
        let mut range_values_iter = range_str
            .split('=')
            .nth(1)
            .unwrap()
            // -10..-5
            .split("..")
            .map(|r| r.parse::<isize>().unwrap());
        range_values_iter.next().unwrap()..=range_values_iter.next().unwrap()
    });
    (ranges_iter.next().unwrap(), ranges_iter.next().unwrap())
}

fn try_land_probe(
    mut velocity: (isize, isize),
    (target_x, target_y): &(RangeInclusive<isize>, RangeInclusive<isize>),
) -> Option<isize> {
    let mut position = (0, 0);

    let mut max_y = 0;

    for _ in 0..200 {
        // move
        position = (position.0 + velocity.0, position.1 + velocity.1);
        if position.1 > max_y {
            max_y = position.1;
        }

        // drag, gravity
        velocity = (velocity.0 - velocity.0.signum(), velocity.1 - 1);

        if target_x.contains(&position.0) && target_y.contains(&position.1) {
            return Some(max_y);
        }
    }

    None
}

pub fn part1(input: &str) -> usize {
    let target_area = parse_input(input);

    let mut max_y = 0;

    for y in -100..100 {
        for x in -100..100 {
            let start_velocity = (x, y);
            if let Some(this_max_y) = try_land_probe(start_velocity, &target_area) {
                max_y = max_y.max(this_max_y as usize);
            }
        }
    }

    max_y
}

use rayon::prelude::*;

pub fn part2(input: &str) -> usize {
    let target_area = parse_input(input);

    (-1000..1000)
        .into_par_iter()
        .map(|y| {
            (-1000..1000)
                .into_par_iter()
                .map(|x| match try_land_probe((x, y), &target_area) {
                    Some(_) => 1,
                    None => 0,
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}
