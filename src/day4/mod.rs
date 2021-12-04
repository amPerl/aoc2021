mod bingo_board;
use bingo_board::BingoBoard;

fn parse_input(input: &str) -> (Vec<usize>, Vec<BingoBoard>) {
    let mut words = input.split_whitespace();

    let drawn_numbers_str = words.next().unwrap();
    let drawn_numbers: Vec<usize> = drawn_numbers_str
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let board_numbers: Vec<usize> = words.map(|s| s.parse::<usize>().unwrap()).collect();

    let board_rows: Vec<[usize; 5]> = board_numbers
        .chunks_exact(5)
        .map(|row_slice| row_slice.try_into().unwrap())
        .collect();

    let boards: Vec<BingoBoard> = board_rows
        .chunks_exact(5)
        .map(|board_slice| board_slice.try_into().unwrap())
        .map(|board_arr: [[usize; 5]; 5]| board_arr.into())
        .collect();

    (drawn_numbers, boards)
}

pub fn part1(input: &str) -> usize {
    let (drawn_numbers, mut boards) = parse_input(input);

    for drawn_number in drawn_numbers {
        for board in boards.iter_mut() {
            if board.draw(drawn_number) {
                return board.sum() * drawn_number;
            }
        }
    }

    unreachable!()
}

pub fn part2(input: &str) -> usize {
    let (drawn_numbers, mut boards) = parse_input(input);

    let mut last_win_score = 0;
    for drawn_number in drawn_numbers {
        for board in boards.iter_mut() {
            if !board.won() && board.draw(drawn_number) {
                last_win_score = board.sum() * drawn_number;
            }
        }
    }

    last_win_score
}
