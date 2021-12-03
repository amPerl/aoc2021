#[test]
fn part1_example() {
    assert_eq!(
        198,
        aoc2021::day3::part1(include_str!("./day3.example.input.txt"))
    );
}

#[test]
fn part1() {
    assert_eq!(
        3912944,
        aoc2021::day3::part1(include_str!("./day3.input.txt"))
    );
}

#[test]
fn part2_example() {
    assert_eq!(
        230,
        aoc2021::day3::part2(include_str!("./day3.example.input.txt"))
    );
}

#[test]
fn part2() {
    assert_eq!(
        4996233,
        aoc2021::day3::part2(include_str!("./day3.input.txt"))
    );
}
