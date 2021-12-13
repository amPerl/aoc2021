#[test]
fn part1_example() {
    assert_eq!(
        17,
        aoc2021::day13::part1(&std::fs::read_to_string("tests/day13/example.txt").unwrap())
    );
}

#[test]
fn part1() {
    assert_eq!(
        788,
        aoc2021::day13::part1(&std::fs::read_to_string("tests/day13/input.txt").unwrap())
    );
}

#[test]
fn part2() {
    let expected: Vec<String> = vec![
        "#  #   ## ###  #  # #### #  # ###   ## ".into(),
        "# #     # #  # # #  #    #  # #  # #  #".into(),
        "##      # ###  ##   ###  #  # ###  #   ".into(),
        "# #     # #  # # #  #    #  # #  # # ##".into(),
        "# #  #  # #  # # #  #    #  # #  # #  #".into(),
        "#  #  ##  ###  #  # ####  ##  ###   ###".into(),
    ];

    let result = aoc2021::day13::part2(&std::fs::read_to_string("tests/day13/input.txt").unwrap());

    for (line_idx, (expected, result)) in expected.into_iter().zip(result.into_iter()).enumerate() {
        assert_eq!(expected, result, "line {}", line_idx);
    }
}
