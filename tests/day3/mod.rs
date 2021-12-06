#[test]
fn part1_example() {
    assert_eq!(
        198,
        aoc2021::day3::part1(&std::fs::read_to_string("tests/day3/example.txt").unwrap())
    );
}

#[test]
fn part1() {
    assert_eq!(
        3912944,
        aoc2021::day3::part1(&std::fs::read_to_string("tests/day3/input.txt").unwrap())
    );
}

#[test]
fn part2_example() {
    assert_eq!(
        230,
        aoc2021::day3::part2(&std::fs::read_to_string("tests/day3/example.txt").unwrap())
    );
}

#[test]
fn part2() {
    assert_eq!(
        4996233,
        aoc2021::day3::part2(&std::fs::read_to_string("tests/day3/input.txt").unwrap())
    );
}
