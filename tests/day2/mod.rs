#[test]
fn part1_example() {
    assert_eq!(150, aoc2021::day2::part1(include_str!("./example.txt")));
}

#[test]
fn part1() {
    assert_eq!(2150351, aoc2021::day2::part1(include_str!("./input.txt")));
}

#[test]
fn part2_example() {
    assert_eq!(900, aoc2021::day2::part2(include_str!("./example.txt")));
}

#[test]
fn part2() {
    assert_eq!(
        1842742223,
        aoc2021::day2::part2(include_str!("./input.txt"))
    );
}
