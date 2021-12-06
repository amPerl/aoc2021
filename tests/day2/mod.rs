#[test]
fn part1_example() {
    assert_eq!(
        150,
        aoc2021::day2::part1(&std::fs::read_to_string("tests/day2/example.txt").unwrap())
    );
}

#[test]
fn part1() {
    assert_eq!(
        2150351,
        aoc2021::day2::part1(&std::fs::read_to_string("tests/day2/input.txt").unwrap())
    );
}

#[test]
fn part2_example() {
    assert_eq!(
        900,
        aoc2021::day2::part2(&std::fs::read_to_string("tests/day2/example.txt").unwrap())
    );
}

#[test]
fn part2() {
    assert_eq!(
        1842742223,
        aoc2021::day2::part2(&std::fs::read_to_string("tests/day2/input.txt").unwrap())
    );
}
