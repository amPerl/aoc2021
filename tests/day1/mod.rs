#[test]
fn part1_example() {
    assert_eq!(7, aoc2021::day1::part1(include_str!("./example.txt")));
}

#[test]
fn part1() {
    assert_eq!(1167, aoc2021::day1::part1(include_str!("./input.txt")));
}

#[test]
fn part2_example() {
    assert_eq!(5, aoc2021::day1::part2(include_str!("./example.txt")));
}

#[test]
fn part2() {
    assert_eq!(1130, aoc2021::day1::part2(include_str!("./input.txt")));
}
