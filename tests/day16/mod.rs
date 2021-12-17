#[test]
fn part1_examples() {
    let examples_raw = std::fs::read_to_string("tests/day16/examples1.txt").unwrap();

    let examples = examples_raw.split_whitespace().map(|line| {
        let mut line_split = line.split(',');
        (
            line_split.next().unwrap(),
            line_split.next().unwrap().parse::<usize>().unwrap(),
        )
    });

    for (input, expected) in examples {
        assert_eq!(expected, aoc2021::day16::part1(input));
    }
}

#[test]
fn part1() {
    assert_eq!(
        934,
        aoc2021::day16::part1(&std::fs::read_to_string("tests/day16/input.txt").unwrap())
    );
}

#[test]
fn part2_examples() {
    let examples_raw = std::fs::read_to_string("tests/day16/examples2.txt").unwrap();

    let examples = examples_raw.split_whitespace().map(|line| {
        let mut line_split = line.split(',');
        (
            line_split.next().unwrap(),
            line_split.next().unwrap().parse::<usize>().unwrap(),
        )
    });

    for (input, expected) in examples {
        assert_eq!(expected, aoc2021::day16::part2(input));
    }
}

#[test]
fn part2() {
    assert_eq!(
        912901337844,
        aoc2021::day16::part2(&std::fs::read_to_string("tests/day16/input.txt").unwrap())
    );
}
