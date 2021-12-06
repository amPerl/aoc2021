#[test]
fn part1_example() {
    assert_eq!(
        7,
        aoc2021::day1::part1(&std::fs::read_to_string("tests/day1/example.txt").unwrap())
    );
}

#[test]
fn part1() {
    assert_eq!(
        1167,
        aoc2021::day1::part1(&std::fs::read_to_string("tests/day1/input.txt").unwrap())
    );
}

#[test]
fn part2_example() {
    assert_eq!(
        5,
        aoc2021::day1::part2(&std::fs::read_to_string("tests/day1/example.txt").unwrap())
    );
}

#[test]
fn part2() {
    assert_eq!(
        1130,
        aoc2021::day1::part2(&std::fs::read_to_string("tests/day1/input.txt").unwrap())
    );
}
