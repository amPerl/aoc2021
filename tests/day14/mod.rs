#[test]
fn part1_example() {
    assert_eq!(
        1588,
        aoc2021::day14::part1(&std::fs::read_to_string("tests/day14/example.txt").unwrap())
    );
}

#[test]
fn part1() {
    assert_eq!(
        2621,
        aoc2021::day14::part1(&std::fs::read_to_string("tests/day14/input.txt").unwrap())
    );
}

#[test]
fn part2_example() {
    assert_eq!(
        2188189693529,
        aoc2021::day14::part2(&std::fs::read_to_string("tests/day14/example.txt").unwrap())
    );
}

#[test]
fn part2() {
    assert_eq!(
        2843834241366,
        aoc2021::day14::part2(&std::fs::read_to_string("tests/day14/input.txt").unwrap())
    );
}
