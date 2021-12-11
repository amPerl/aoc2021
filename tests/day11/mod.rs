#[test]
fn simulate_flash() {
    let mut grid = [
        [1, 1, 1, 1, 1],
        [1, 9, 9, 9, 1],
        [1, 9, 1, 9, 1],
        [1, 9, 9, 9, 1],
        [1, 1, 1, 1, 1],
    ];
    let expected = [
        [3, 4, 5, 4, 3],
        [4, 0, 0, 0, 4],
        [5, 0, 0, 0, 5],
        [4, 0, 0, 0, 4],
        [3, 4, 5, 4, 3],
    ];
    aoc2021::day11::simulate_step(&mut grid);
    for y in 0..grid.len() {
        assert_eq!(grid[y], expected[y], "y {} mismatch", y);
    }
}

#[test]
fn part1_example() {
    assert_eq!(
        1656,
        aoc2021::day11::part1(&std::fs::read_to_string("tests/day11/example.txt").unwrap())
    );
}

#[test]
fn part1() {
    assert_eq!(
        1644,
        aoc2021::day11::part1(&std::fs::read_to_string("tests/day11/input.txt").unwrap())
    );
}

#[test]
fn part2_example() {
    assert_eq!(
        195,
        aoc2021::day11::part2(&std::fs::read_to_string("tests/day11/example.txt").unwrap())
    );
}

#[test]
fn part2() {
    assert_eq!(
        229,
        aoc2021::day11::part2(&std::fs::read_to_string("tests/day11/input.txt").unwrap())
    );
}
