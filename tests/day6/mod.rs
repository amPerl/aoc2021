#[test]
fn part1_example() {
    assert_eq!(5934, aoc2021::day6::part1(include_str!("./example.txt")));
}

#[test]
fn part1() {
    assert_eq!(360761, aoc2021::day6::part1(include_str!("./input.txt")));
}

#[test]
fn part2_example() {
    assert_eq!(
        26984457539,
        aoc2021::day6::part2(include_str!("./example.txt"))
    );
}

#[test]
fn part2() {
    assert_eq!(
        1632779838045,
        aoc2021::day6::part2(include_str!("./input.txt"))
    );
}
