#[test]
fn part1_example() {
    assert_eq!(
        40,
        aoc2021::day15::part1(&std::fs::read_to_string("tests/day15/example.txt").unwrap())
    );
}

#[test]
fn part1() {
    assert_eq!(
        714,
        aoc2021::day15::part1(&std::fs::read_to_string("tests/day15/input.txt").unwrap())
    );
}

#[test]
fn part2_example() {
    assert_eq!(
        315,
        aoc2021::day15::part2(&std::fs::read_to_string("tests/day15/example.txt").unwrap())
    );
}

#[test]
fn part2() {
    assert_eq!(
        2948,
        aoc2021::day15::part2(&std::fs::read_to_string("tests/day15/input.txt").unwrap())
    );
}
