use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .split_whitespace()
        .map(|row| {
            row.chars()
                .flat_map(|ch| ch.to_digit(10))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn get_neighbour_coords(x: usize, y: usize, width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();

    let left_x = x as isize - 1;
    if left_x >= 0 {
        neighbours.push((left_x as usize, y));
    }
    let right_x = x as isize + 1;
    if right_x < width as isize {
        neighbours.push((right_x as usize, y));
    }
    let up_y = y as isize - 1;
    if up_y >= 0 {
        neighbours.push((x, up_y as usize));
    }
    let down_y = y as isize + 1;
    if down_y < height as isize {
        neighbours.push((x, down_y as usize));
    }

    neighbours
}

#[allow(clippy::ptr_arg)]
fn find_sinks(grid: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut sinks = Vec::new();
    let height = grid.len();
    let width = grid[0].len();

    for (y, sink_row) in grid.iter().enumerate() {
        for (x, sink_val) in sink_row.iter().enumerate() {
            if get_neighbour_coords(x, y, width, height)
                .into_iter()
                .map(|(nx, ny)| grid[ny][nx])
                .filter(|nval| *nval <= *sink_val)
                .count()
                == 0
            {
                sinks.push((x, y));
            }
        }
    }

    sinks
}

pub fn part1(input: &str) -> usize {
    let grid = parse_input(input);

    find_sinks(&grid)
        .into_iter()
        .map(|(x, y)| grid[y][x] + 1)
        .sum::<u32>() as usize
}

#[allow(clippy::ptr_arg)]
fn visit_basin(grid: &Vec<Vec<u32>>, x: usize, y: usize, visited: &mut HashSet<(usize, usize)>) {
    let here = (x, y);

    if visited.contains(&here) {
        return;
    }

    visited.insert((x, y));

    let height = grid.len();
    let width = grid[0].len();

    for (nx, ny) in get_neighbour_coords(x, y, width, height) {
        if grid[ny][nx] != 9 {
            visit_basin(grid, nx, ny, visited);
        }
    }
}

pub fn part2(input: &str) -> usize {
    let grid = parse_input(input);

    let mut basins = Vec::new();

    for (sink_x, sink_y) in find_sinks(&grid) {
        let mut visited = HashSet::new();
        visit_basin(&grid, sink_x, sink_y, &mut visited);
        basins.push(visited.len());
    }

    basins.sort_unstable();
    basins.into_iter().rev().take(3).product()
}
