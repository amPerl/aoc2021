#[test]
fn part1_example() {
    assert_eq!(
        15,
        aoc2021::day9::part1(&std::fs::read_to_string("tests/day9/example.txt").unwrap())
    );
}

#[test]
fn part1() {
    assert_eq!(
        564,
        aoc2021::day9::part1(&std::fs::read_to_string("tests/day9/input.txt").unwrap())
    );
}

#[test]
fn part2_example() {
    assert_eq!(
        1134,
        aoc2021::day9::part2(&std::fs::read_to_string("tests/day9/example.txt").unwrap())
    );
}

#[test]
fn part2() {
    assert_eq!(
        1038240,
        aoc2021::day9::part2(&std::fs::read_to_string("tests/day9/input.txt").unwrap())
    );
}
