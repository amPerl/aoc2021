use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct BingoBoard {
    rows: Vec<HashSet<usize>>,
    columns: Vec<HashSet<usize>>,
    won: bool,
}

impl From<[[usize; 5]; 5]> for BingoBoard {
    fn from(row_major: [[usize; 5]; 5]) -> BingoBoard {
        let columns = (0..5)
            .map(|column_idx| {
                row_major
                    .iter()
                    .map(|row_slice| row_slice[column_idx])
                    .collect()
            })
            .collect();

        let rows = row_major
            .map(|row_arr| row_arr.into_iter().collect())
            .into();

        BingoBoard {
            rows,
            columns,
            won: false,
        }
    }
}

impl BingoBoard {
    pub fn draw(&mut self, number: usize) -> bool {
        for row in self.rows.iter_mut() {
            row.remove(&number);
            if row.is_empty() {
                self.won = true;
            }
        }

        for column in self.columns.iter_mut() {
            column.remove(&number);
            if column.is_empty() {
                self.won = true;
            }
        }

        self.won
    }

    pub fn won(&self) -> bool {
        self.won
    }

    pub fn sum(&self) -> usize {
        self.rows
            .iter()
            .map::<usize, _>(|row| row.iter().sum())
            .sum()
    }
}
