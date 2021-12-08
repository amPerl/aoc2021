#[test]
fn part1_example() {
    assert_eq!(
        26,
        aoc2021::day8::part1(&std::fs::read_to_string("tests/day8/example.txt").unwrap())
    );
}

#[test]
fn part1() {
    assert_eq!(
        521,
        aoc2021::day8::part1(&std::fs::read_to_string("tests/day8/input.txt").unwrap())
    );
}

#[test]
fn part2_example() {
    assert_eq!(
        61229,
        aoc2021::day8::part2(&std::fs::read_to_string("tests/day8/example.txt").unwrap())
    );
}

#[test]
fn part2() {
    assert_eq!(
        1016804,
        aoc2021::day8::part2(&std::fs::read_to_string("tests/day8/input.txt").unwrap())
    );
}
