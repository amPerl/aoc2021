#[test]
fn part1_example() {
    assert_eq!(
        45,
        aoc2021::day17::part1(&std::fs::read_to_string("tests/day17/example.txt").unwrap())
    );
}

#[test]
fn part1() {
    assert_eq!(
        4656,
        aoc2021::day17::part1(&std::fs::read_to_string("tests/day17/input.txt").unwrap())
    );
}

#[test]
fn part2_example() {
    assert_eq!(
        112,
        aoc2021::day17::part2(&std::fs::read_to_string("tests/day17/example.txt").unwrap())
    );
}

#[test]
fn part2() {
    assert_eq!(
        1908,
        aoc2021::day17::part2(&std::fs::read_to_string("tests/day17/input.txt").unwrap())
    );
}
