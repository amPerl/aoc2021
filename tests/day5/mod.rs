#[test]
fn part1_example() {
    assert_eq!(5, aoc2021::day5::part1(include_str!("./example.txt")));
}

#[test]
fn part1() {
    assert_eq!(7297, aoc2021::day5::part1(include_str!("./input.txt")));
}

#[test]
fn part2_example() {
    assert_eq!(12, aoc2021::day5::part2(include_str!("./example.txt")));
}

#[test]
fn part2() {
    assert_eq!(21038, aoc2021::day5::part2(include_str!("./input.txt")));
}
