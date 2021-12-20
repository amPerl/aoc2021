use std::{fmt::Debug, ops::Add, time::Instant};

#[derive(PartialEq)]
pub enum Number {
    Single(usize),
    Pair(Box<Number>, Box<Number>),
}

impl Number {
    fn pair(left: Number, right: Number) -> Number {
        Number::Pair(Box::new(left), Box::new(right))
    }

    fn magnitude(&self) -> usize {
        match self {
            Number::Single(val) => *val,
            Number::Pair(left, right) => left.magnitude() * 3 + right.magnitude() * 2,
        }
    }

    fn should_explode(&self, depth: usize) -> bool {
        match self {
            Number::Pair(left, right) => match (&**left, &**right, depth) {
                (Number::Single(_left_value), Number::Single(_right_value), 4..) => true,
                _ => left.should_explode(depth + 1) || right.should_explode(depth + 1),
            },
            _ => false,
        }
    }

    fn should_split(&self) -> bool {
        match self {
            Number::Single(val) => *val > 10,
            Number::Pair(left, right) => left.should_split() || right.should_split(),
        }
    }

    fn carry(&self, (to_left, to_right): (usize, usize)) -> Self {
        // if to_left > 0 || to_right > 0 {
        //     eprintln!("carry {:?}, {:?}", self, (to_left, to_right));
        // }
        match self {
            Number::Single(value) => Number::Single(*value),
            Number::Pair(left, right) => match (&**left, &**right) {
                (Number::Single(left_value), Number::Single(right_value)) => Number::pair(
                    Number::Single(left_value + to_left),
                    Number::Single(right_value + to_right),
                ),
                (Number::Single(left_value), right) => Number::pair(
                    Number::Single(left_value + to_left),
                    right.carry((0, to_right)),
                ),
                (left, Number::Single(right_value)) => Number::pair(
                    left.carry((to_left, 0)),
                    Number::Single(right_value + to_right),
                ),
                (left, right) => Number::pair(left.carry((to_left, 0)), right.carry((0, to_right))),
            },
        }
    }

    pub fn explode_mut(&mut self, depth: usize) -> (usize, usize) {
        // eprintln!("exploding {:?}, depth: {}", self, depth);
        let mut exploded = false;
        let carries = match self {
            Number::Single(_) => (0, 0),
            Number::Pair(left, right) => match (&mut **left, &mut **right, depth) {
                (Number::Single(left_value), Number::Single(right_value), 4..) => {
                    // eprintln!(
                    //     "depth > 3 singles pair, BOOM. carrying: {:?}",
                    //     (left_value, right_value)
                    // );
                    exploded = true;
                    (*left_value, *right_value)
                }
                (Number::Single(left_value), right, _) => {
                    let right_carry = right.explode_mut(depth + 1);
                    if right_carry.0 > 0 {
                        *left = Box::new(Number::Single(*left_value + right_carry.0));
                    }
                    (0, right_carry.1)
                }
                (left, Number::Single(right_value), _) => {
                    let left_carry = left.explode_mut(depth + 1);
                    if left_carry.1 > 0 {
                        *right = Box::new(Number::Single(*right_value + left_carry.1));
                    }
                    (left_carry.0, 0)
                }
                (_, _, _) => {
                    let left_carry = left.explode_mut(depth + 1);
                    let right_carry = right.explode_mut(depth + 1);
                    if right_carry.0 > 0 {
                        *left = Box::new(left.carry((0, right_carry.0)));
                    }
                    if left_carry.1 > 0 {
                        *right = Box::new(right.carry((left_carry.1, 0)));
                    }
                    // eprintln!(
                    //     "handling pair:\n\tleft: {:?}\n\t\tleft_carry: {:?}\n\tright: {:?}\n\t\tright_carry {:?}",
                    //     new_left, left_carry, new_right, right_carry
                    // );
                    (left_carry.0 + right_carry.0, left_carry.1 + right_carry.1)
                }
            },
        };

        if exploded {
            *self = Number::Single(0);
        }

        carries
    }

    pub fn explode(self, depth: usize) -> (Self, (usize, usize)) {
        // eprintln!("exploding {:?}, depth: {}", self, depth);
        match self {
            Number::Single(_) => (self, (0, 0)),
            Number::Pair(left, right) => match (*left, *right, depth) {
                (Number::Single(left_value), Number::Single(right_value), 4..) => {
                    // eprintln!(
                    //     "depth > 3 singles pair, BOOM. carrying: {:?}",
                    //     (left_value, right_value)
                    // );
                    (Number::Single(0), (left_value, right_value))
                }
                (Number::Single(left_value), right, _) => {
                    let (new_right, right_carry) = right.explode(depth + 1);
                    (
                        Number::pair(Number::Single(left_value + right_carry.0), new_right),
                        (0, right_carry.1),
                    )
                }
                (left, Number::Single(right_value), _) => {
                    let (new_left, left_carry) = left.explode(depth + 1);
                    (
                        Number::pair(new_left, Number::Single(right_value + left_carry.1)),
                        (left_carry.0, 0),
                    )
                }
                (left, right, _) => {
                    let (new_left, left_carry) = left.explode(depth + 1);
                    let (new_right, right_carry) = right.explode(depth + 1);
                    // eprintln!(
                    //     "handling pair:\n\tleft: {:?}\n\t\tleft_carry: {:?}\n\tright: {:?}\n\t\tright_carry {:?}",
                    //     new_left, left_carry, new_right, right_carry
                    // );
                    (
                        Number::pair(
                            new_left.carry((0, right_carry.0)),
                            new_right.carry((left_carry.1, 0)),
                        ),
                        (left_carry.0 + right_carry.0, left_carry.1 + right_carry.1),
                    )
                }
            },
        }
    }

    pub fn split(self) -> Self {
        match self {
            Number::Single(val) => {
                if val >= 10 {
                    let half = val as f64 / 2.0;
                    Number::pair(
                        Number::Single(half.floor() as usize).split(),
                        Number::Single(half.ceil() as usize).split(),
                    )
                } else {
                    self
                }
            }
            Number::Pair(left, right) => Number::pair(left.split(), right.split()),
        }
    }

    pub fn reduce(self) -> Self {
        // eprintln!("reducing {:?}", self);
        let mut result = self;
        loop {
            let (mut exploded, mut split) = (false, false);

            let now = Instant::now();
            loop {
                if !result.should_explode(0) {
                    // eprintln!("done exploding");
                    break;
                }

                // eprintln!("exploding \t{:?}", result);
                result.explode_mut(0);
                // eprintln!("\tgot \t{:?}", result);
                exploded = true;
            }
            if exploded {
                eprintln!("finished explode pass, {:?}us", now.elapsed().as_micros());
            }

            let now = Instant::now();
            loop {
                if !result.should_split() {
                    // eprintln!("done splitting");
                    break;
                }

                // eprintln!("splitting \t{:?}", result);
                result = result.split();
                // eprintln!("\tgot \t{:?}", result);
                split = true;
            }
            if split {
                eprintln!("finished split pass, {:?}us", now.elapsed().as_micros());
            }

            if !exploded && !split {
                return result;
            }
        }
    }
}

impl Add for Number {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Number::pair(self, other).reduce()
    }
}

impl Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single(val) => f.write_fmt(format_args!("{}", val)),
            Self::Pair(left, right) => f.write_fmt(format_args!("[{:?},{:?}]", left, right)),
        }
    }
}

pub fn parse_number(input: &mut impl Iterator<Item = char>) -> Number {
    let first = input.next().unwrap();
    if first == '[' {
        let left = parse_number(input);
        let right = parse_number(input);
        let _closing_bracket = input.next();
        Number::pair(left, right)
    } else {
        Number::Single(
            std::iter::once(first)
                .chain(input.take_while(|&ch| ch != ']' && ch != ','))
                .collect::<String>()
                .parse()
                .unwrap(),
        )
    }
}

fn parse_input(input: &str) -> Vec<Number> {
    input
        .split_whitespace()
        .map(|line| parse_number(&mut line.chars()))
        .collect()
}

pub fn part1(input: &str) -> usize {
    let mut input_iter = parse_input(input).into_iter();

    let mut sum = input_iter.next().unwrap();
    for number in input_iter {
        // eprintln!("{:?} + {:?} =", sum, number);
        sum = sum + number;
        // eprintln!("{:?}", sum);
    }

    sum.magnitude()
}

pub fn part2(_input: &str) -> usize {
    0
}
