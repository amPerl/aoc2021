#[test]
fn part1_example() {
    assert_eq!(
        26397,
        aoc2021::day10::part1(&std::fs::read_to_string("tests/day10/example.txt").unwrap())
    );
}

#[test]
fn part1() {
    assert_eq!(
        168417,
        aoc2021::day10::part1(&std::fs::read_to_string("tests/day10/input.txt").unwrap())
    );
}

#[test]
fn part2_example() {
    assert_eq!(
        288957,
        aoc2021::day10::part2(&std::fs::read_to_string("tests/day10/example.txt").unwrap())
    );
}

#[test]
fn part2() {
    assert_eq!(
        2802519786,
        aoc2021::day10::part2(&std::fs::read_to_string("tests/day10/input.txt").unwrap())
    );
}
