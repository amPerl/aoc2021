use std::collections::HashSet;

fn parse_grid<const W: usize, const H: usize>(input: &str) -> [[u32; W]; H] {
    input
        .split_whitespace()
        .map(|line| {
            line.chars()
                .flat_map(|x| x.to_digit(10))
                .collect::<Vec<_>>()
                .as_slice()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<_>>()
        .as_slice()
        .try_into()
        .unwrap()
}

fn get_valid_neighbours<const W: usize, const H: usize>(
    x: usize,
    y: usize,
    _grid: &[[u32; W]; H],
) -> Vec<(usize, usize)> {
    let xi = x as isize;
    let yi = y as isize;

    (-1_isize..=1)
        .map(|y_off| y_off + yi)
        .filter(|&ny| ny >= 0 && ny < H as isize)
        .flat_map(|ny| {
            (-1_isize..=1)
                .map(|x_off| x_off + xi)
                .filter(|&nx| nx >= 0 && nx < W as isize)
                .filter(move |&nx| !(nx == xi && ny == yi))
                .map(move |nx| (nx as usize, ny as usize))
        })
        .collect()
}

pub fn simulate_step<const W: usize, const H: usize>(grid: &mut [[u32; W]; H]) -> usize {
    for row in grid.iter_mut() {
        for cell in row.iter_mut() {
            *cell += 1;
        }
    }

    let mut flashed_this_step = HashSet::new();

    loop {
        let mut flash_victims = Vec::new();

        for y in 0..H {
            for x in 0..W {
                if grid[y][x] <= 9 || flashed_this_step.contains(&(x, y)) {
                    continue;
                }

                flashed_this_step.insert((x, y));

                for (nx, ny) in get_valid_neighbours(x, y, grid).into_iter() {
                    flash_victims.push((nx, ny));
                }
            }
        }

        if flash_victims.is_empty() {
            break;
        }

        for (x, y) in flash_victims.drain(..) {
            grid[y][x] += 1;
        }

        for (x, y) in flashed_this_step.iter() {
            grid[*y][*x] = 0;
        }
    }

    flashed_this_step.len()
}

pub fn part1(input: &str) -> usize {
    let mut grid: [[u32; 10]; 10] = parse_grid(input);

    let mut total_flashes = 0;

    for _ in 0..100 {
        total_flashes += simulate_step(&mut grid);
    }

    total_flashes
}

pub fn part2(input: &str) -> usize {
    let mut grid: [[u32; 10]; 10] = parse_grid(input);

    for i in 0.. {
        if simulate_step(&mut grid) == 100 {
            return i + 1;
        }
    }

    unreachable!()
}
