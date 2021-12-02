use itertools::{Chunks, Itertools};
use std::str::SplitWhitespace;

struct ParsedInputIterator<'a> {
    lines_iter: Chunks<'a, SplitWhitespace<'a>>,
}

impl<'a> Iterator for ParsedInputIterator<'a> {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        let mut line_words = self.lines_iter.next()?;
        let direction = line_words.next()?;
        let val = line_words.next()?.parse::<isize>().unwrap();
        Some(match direction {
            "forward" => (val, 0),
            "down" => (0, val),
            "up" => (0, -val),
            _ => unreachable!(),
        })
    }
}

pub fn part1(input: &str) -> usize {
    let (x, y) = ParsedInputIterator {
        lines_iter: input.split_whitespace().chunks(2).into_iter(),
    }
    .reduce(|(x, y), (dx, dy)| (x + dx, y + dy))
    .unwrap();
    (x * y) as usize
}

pub fn part2(input: &str) -> usize {
    let (_aim, x, y) = ParsedInputIterator {
        lines_iter: input.split_whitespace().chunks(2).into_iter(),
    }
    .fold((0, 0, 0), |(aim, x, y), (dx, daim)| {
        (aim + daim, x + dx, y + dx * aim)
    });
    (x * y) as usize
}
