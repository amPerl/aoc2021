#[test]
fn part1_example() {
    assert_eq!(
        10,
        aoc2021::day12::part1(&std::fs::read_to_string("tests/day12/example.txt").unwrap())
    );
}

#[test]
fn part1() {
    assert_eq!(
        4707,
        aoc2021::day12::part1(&std::fs::read_to_string("tests/day12/input.txt").unwrap())
    );
}

#[test]
fn part2_example() {
    assert_eq!(
        36,
        aoc2021::day12::part2(&std::fs::read_to_string("tests/day12/example.txt").unwrap())
    );
}

#[test]
fn part2() {
    assert_eq!(
        130493,
        aoc2021::day12::part2(&std::fs::read_to_string("tests/day12/input.txt").unwrap())
    );
}
