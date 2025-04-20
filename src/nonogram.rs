pub struct Nonogram {
    pub width: usize,
    pub height: usize,
    pub board: Vec<Vec<bool>>,
    pub column_hints: Vec<Vec<Hint>>,
    pub row_hints: Vec<Vec<Hint>>,
}

impl Nonogram {
    pub fn new(column_hints: Vec<Vec<Hint>>, row_hints: Vec<Vec<Hint>>) -> Nonogram {
        let width = column_hints.len();
        let height = row_hints.len();
        let board = vec![vec![false; height]; width];
        Nonogram {
            width,
            height,
            board,
            column_hints,
            row_hints,
        }
    }

    pub fn set_square(&mut self, row: usize, column: usize, filled_in: bool) {
        self.board[row][column] = filled_in;
    }

    pub fn solve(&self) -> Nonogram {
        todo!("Implement solver");
    }

    pub fn draw_board_to_console(&self) {
        let max_column_hint_depth = self
            .column_hints
            .iter()
            .map(|column| column.len())
            .max()
            .unwrap_or(0);
        let max_row_hint_depth = self
            .row_hints
            .iter()
            .map(|row| row.len())
            .max()
            .unwrap_or(0);
        for hint_row in 0..max_column_hint_depth {
            for _ in 0..max_row_hint_depth {
                print!("  ");
            }
            print!("   ");
            let hint_row_inverse = max_column_hint_depth - hint_row;
            let mut printable_rows = self
                .column_hints
                .iter()
                .enumerate()
                .filter(|(_, column_hint)| column_hint.len() >= hint_row_inverse)
                .collect::<Vec<_>>();
            printable_rows.sort_by(|a, b| a.0.cmp(&b.0));
            let mut cursor_pos = 0;
            for (index, column_hint) in printable_rows {
                for _ in cursor_pos..index {
                    print!("   ");
                }
                print!(" {} ", column_hint[hint_row_inverse - 1].hint);
                cursor_pos = index + 1;
            }
            println!();
        }
        for hint_column in 0..self.row_hints.len() {
            let depth_difference = max_row_hint_depth - self.row_hints[hint_column].len();
            for _ in 0..depth_difference {
                print!("   ");
            }
            for hint in &self.row_hints[hint_column] {
                print!(" {} ", hint.hint)
            }
            for board_space in &self.board[hint_column] {
                if *board_space {
                    print!("[@]");
                } else {
                    print!("[ ]");
                }
            }
            println!();
        }
    }
}

#[derive(Clone)]
pub struct Hint {
    pub hint: i32,
    pub fulfilled: bool,
}

impl Hint {
    pub fn new(hint: i32) -> Hint {
        Hint {
            hint,
            fulfilled: false,
        }
    }
}
