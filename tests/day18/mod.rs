#[test]
fn number_explode_1() {
    let number = aoc2021::day18::parse_number(&mut "[[[[[9,8],1],2],3],4]".chars());
    let expected = aoc2021::day18::parse_number(&mut "[[[[0,9],2],3],4]".chars());
    let (exploded, _carry) = number.explode(0);
    assert_eq!(expected, exploded);
}

#[test]
fn number_explode_2() {
    let number = aoc2021::day18::parse_number(&mut "[7,[6,[5,[4,[3,2]]]]]".chars());
    let expected = aoc2021::day18::parse_number(&mut "[7,[6,[5,[7,0]]]]".chars());
    let (exploded, _carry) = number.explode(0);
    assert_eq!(expected, exploded);
}

#[test]
fn number_explode_3() {
    let number = aoc2021::day18::parse_number(&mut "[[6,[5,[4,[3,2]]]],1]".chars());
    let expected = aoc2021::day18::parse_number(&mut "[[6,[5,[7,0]]],3]".chars());
    let (exploded, _carry) = number.explode(0);
    assert_eq!(expected, exploded);
}

#[test]
fn number_explode_4() {
    let number = aoc2021::day18::parse_number(&mut "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".chars());
    let expected = aoc2021::day18::parse_number(&mut "[[3,[2,[8,0]]],[9,[5,[7,0]]]]".chars());
    let (exploded, _carry) = number.explode(0);
    assert_eq!(expected, exploded);
}

#[test]
fn number_reduce_1() {
    let number = aoc2021::day18::parse_number(&mut "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".chars());
    let expected = aoc2021::day18::parse_number(&mut "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".chars());
    assert_eq!(expected, number.reduce());
}

#[test]
fn part1_example() {
    assert_eq!(
        4140,
        aoc2021::day18::part1(&std::fs::read_to_string("tests/day18/example.txt").unwrap())
    );
}

#[test]
fn part1() {
    assert_eq!(
        0,
        aoc2021::day18::part1(&std::fs::read_to_string("tests/day18/input.txt").unwrap())
    );
}

#[test]
fn part2_example() {
    assert_eq!(
        0,
        aoc2021::day18::part2(&std::fs::read_to_string("tests/day18/example.txt").unwrap())
    );
}

#[test]
fn part2() {
    assert_eq!(
        0,
        aoc2021::day18::part2(&std::fs::read_to_string("tests/day18/input.txt").unwrap())
    );
}
