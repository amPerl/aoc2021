use std::collections::BinaryHeap;

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .split_whitespace()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

#[derive(PartialEq, Eq)]
struct Node {
    position: (usize, usize),
    cost: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn find_path(grid: &[Vec<u8>]) -> Option<usize> {
    let height = grid.len();
    let width = grid[0].len();

    let start = (0, 0);
    let target = (width - 1, height - 1);

    let mut distances = vec![vec![usize::MAX; width]; height];
    distances[0][0] = grid[0][0] as usize;

    let mut unvisited = BinaryHeap::new();
    unvisited.push(Node {
        position: start,
        cost: 0,
    });

    while let Some(Node {
        position: pos,
        cost,
    }) = unvisited.pop()
    {
        if pos == target {
            return Some(cost);
        }

        if cost > distances[pos.1][pos.0] {
            continue;
        }

        let neighbours = [
            (pos.0 as isize - 1, pos.1 as isize),
            (pos.0 as isize + 1, pos.1 as isize),
            (pos.0 as isize, pos.1 as isize - 1),
            (pos.0 as isize, pos.1 as isize + 1),
        ]
        .into_iter()
        .filter(|npos| {
            npos.0 >= 0 && npos.1 >= 0 && npos.0 < width as isize && npos.1 < height as isize
        })
        .map(|npos| (npos.0 as usize, npos.1 as usize));

        for npos in neighbours {
            let edge_cost = grid[npos.1][npos.0] as usize;
            let ncost = cost + edge_cost;

            if ncost < distances[npos.1][npos.0] {
                unvisited.push(Node {
                    position: npos,
                    cost: ncost,
                });
                distances[npos.1][npos.0] = ncost;
            }
        }
    }

    None
}

pub fn part1(input: &str) -> usize {
    find_path(&parse_input(input)).unwrap()
}

pub fn part2(input: &str) -> usize {
    let orig = parse_input(input);
    let orig_h = orig.len();
    let orig_w = orig[0].len();

    let mut grid = vec![vec![0u8; orig_w * 5]; orig_h * 5];

    for tile_y in 0..5 {
        let tile_y_offset = tile_y * orig_h;
        for tile_x in 0..5 {
            let tile_x_offset = tile_x * orig_w;
            let tile_cell_offset = tile_x as u8 + tile_y as u8;

            for (orig_y, orig_row) in orig.iter().enumerate() {
                for (orig_x, orig_cell) in orig_row.iter().enumerate() {
                    let grid_y = tile_y_offset + orig_y;
                    let grid_x = tile_x_offset + orig_x;
                    let grid_cell = ((orig_cell - 1) + tile_cell_offset) % 9 + 1;
                    grid[grid_y][grid_x] = grid_cell;
                }
            }
        }
    }

    find_path(&grid).unwrap()
}
