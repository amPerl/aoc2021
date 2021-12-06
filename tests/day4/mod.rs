#[test]
fn part1_example() {
    assert_eq!(
        4512,
        aoc2021::day4::part1(&std::fs::read_to_string("tests/day4/example.txt").unwrap())
    );
}

#[test]
fn part1() {
    assert_eq!(
        11774,
        aoc2021::day4::part1(&std::fs::read_to_string("tests/day4/input.txt").unwrap())
    );
}

#[test]
fn part2_example() {
    assert_eq!(
        1924,
        aoc2021::day4::part2(&std::fs::read_to_string("tests/day4/example.txt").unwrap())
    );
}

#[test]
fn part2() {
    assert_eq!(
        4495,
        aoc2021::day4::part2(&std::fs::read_to_string("tests/day4/input.txt").unwrap())
    );
}
