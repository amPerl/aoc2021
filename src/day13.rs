use std::collections::HashSet;

type Point = (usize, usize);

fn parse_input(input: &str) -> (HashSet<Point>, Vec<Point>) {
    let mut dots = HashSet::new();
    let mut folds = Vec::new();

    for word in input.split_whitespace() {
        if word == "fold" || word == "along" {
            continue;
        }

        if word.contains(',') {
            let mut coords = word.split(',');
            let x = coords.next().unwrap().parse::<usize>().unwrap();
            let y = coords.next().unwrap().parse::<usize>().unwrap();
            dots.insert((x, y));
        } else if word.contains('=') {
            let mut fold = word.split('=');
            let axis = fold.next().unwrap();
            let value = fold.next().unwrap().parse::<usize>().unwrap();
            folds.push(if axis == "y" { (0, value) } else { (value, 0) });
        }
    }

    (dots, folds)
}

fn fold_single_axis(fold: usize, value: usize) -> usize {
    if value < fold {
        value
    } else {
        fold * 2 - value
    }
}

fn fold_both_axes((fold_x, fold_y): Point, dots: HashSet<Point>) -> HashSet<Point> {
    let mut new_dots = HashSet::new();

    for (dot_x, dot_y) in dots.into_iter() {
        if fold_y > 0 {
            new_dots.insert((dot_x, fold_single_axis(fold_y, dot_y)));
        } else if fold_x > 0 {
            new_dots.insert((fold_single_axis(fold_x, dot_x), dot_y));
        }
    }

    new_dots
}

pub fn part1(input: &str) -> usize {
    let (mut dots, folds) = parse_input(input);

    for fold in folds.into_iter().take(1) {
        dots = fold_both_axes(fold, dots);
    }

    dots.len()
}

pub fn part2(input: &str) -> Vec<String> {
    let (mut dots, folds) = parse_input(input);

    for fold in folds.into_iter() {
        dots = fold_both_axes(fold, dots);
    }

    let max_x = dots.iter().fold(0, |max_x, &(x, _)| x.max(max_x));
    let max_y = dots.iter().fold(0, |max_y, &(_, y)| y.max(max_y));

    (0..=max_y)
        .map(|y| {
            (0..=max_x)
                .map(|x| if dots.contains(&(x, y)) { '#' } else { ' ' })
                .collect()
        })
        .collect()
}
