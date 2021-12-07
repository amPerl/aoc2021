#[test]
fn part1_example() {
    assert_eq!(
        37,
        aoc2021::day7::part1(&std::fs::read_to_string("tests/day7/example.txt").unwrap())
    );
}

#[test]
fn part1() {
    assert_eq!(
        339321,
        aoc2021::day7::part1(&std::fs::read_to_string("tests/day7/input.txt").unwrap())
    );
}

#[test]
fn part2_example() {
    assert_eq!(
        168,
        aoc2021::day7::part2(&std::fs::read_to_string("tests/day7/example.txt").unwrap())
    );
}

#[test]
fn part2() {
    assert_eq!(
        95476244,
        aoc2021::day7::part2(&std::fs::read_to_string("tests/day7/input.txt").unwrap())
    );
}
