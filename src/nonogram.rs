use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub struct Nonogram {
    pub width: usize,
    pub height: usize,
    pub board: Vec<Vec<SpaceStatus>>,
    pub column_hints: Vec<Vec<Hint>>,
    pub row_hints: Vec<Vec<Hint>>,
}

impl Nonogram {
    pub fn new(column_hints: Vec<Vec<Hint>>, row_hints: Vec<Vec<Hint>>) -> Nonogram {
        let width = column_hints.len();
        let height = row_hints.len();
        let board = vec![vec![SpaceStatus::Unknown; height]; width];
        Nonogram {
            width,
            height,
            board,
            column_hints,
            row_hints,
        }
    }

    pub fn set_square(&mut self, row: usize, column: usize, fill_status: SpaceStatus) {
        self.board[row][column] = fill_status;
    }

    pub fn solve(&mut self) {
        self.solve_strat_finished_columns();
        self.solve_strat_finished_rows();
    }

    pub fn solve_strat_finished_rows(&mut self) {
        for row_hint_collection in 0..self.row_hints.len() {
            if self.row_hints[row_hint_collection]
                .iter()
                .map(|f| f.hint)
                .sum::<i32>()
                + (self.row_hints[row_hint_collection].len() - 1) as i32
                == self.width.try_into().unwrap()
            {
                let mut row_position = self.width - 1;
                let mut finished_iter = false;
                for hint in 0..self.row_hints[row_hint_collection].len() {
                    let remaining_iterations = self.row_hints[row_hint_collection][hint].hint;
                    for _ in 0..remaining_iterations {
                        self.set_square(row_hint_collection, row_position, SpaceStatus::Filled);
                        if row_position != 0 {
                            row_position -= 1;
                        } else {
                            finished_iter = true;
                        }
                    }
                    if !finished_iter {
                        self.set_square(row_hint_collection, row_position, SpaceStatus::NotFilled);
                    }
                    self.column_hints[row_hint_collection][hint].fulfilled = true;
                    row_position -= 1;
                }
            }
        }
    }

    pub fn solve_strat_finished_columns(&mut self) {
        for column_hint_collection in 0..self.column_hints.len() {
            if self.column_hints[column_hint_collection]
                .iter()
                .map(|f| f.hint)
                .sum::<i32>()
                + (self.column_hints[column_hint_collection].len() - 1) as i32
                == self.height.try_into().unwrap()
            {
                let mut column_position = self.height - 1;
                let mut finished_iter = false;
                for hint in 0..self.column_hints[column_hint_collection].len() {
                    let remaining_iterations = self.column_hints[column_hint_collection][hint].hint;
                    for _ in 0..remaining_iterations {
                        self.set_square(
                            column_position,
                            column_hint_collection,
                            SpaceStatus::Filled,
                        );
                        if column_position != 0 {
                            column_position -= 1;
                        } else {
                            finished_iter = true;
                        }
                    }
                    if !finished_iter {
                        self.set_square(
                            column_position,
                            column_hint_collection,
                            SpaceStatus::NotFilled,
                        );
                    }
                    self.column_hints[column_hint_collection][hint].fulfilled = true;
                    column_position -= 1;
                }
            }
        }
    }

    pub fn draw_board_to_console(&self) {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);

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
                match board_space {
                    SpaceStatus::Filled => {
                        _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)));
                        _ = write!(&mut stdout, "[O]");
                        _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)));
                    }
                    SpaceStatus::NotFilled => {
                        _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)));
                        _ = write!(&mut stdout, "[X]");
                        _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)));
                    }
                    SpaceStatus::Unknown => {
                        _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)));
                        _ = write!(&mut stdout, "[ ]");
                        _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)));
                    }
                };
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

#[derive(Clone)]
pub enum SpaceStatus {
    Filled,
    NotFilled,
    Unknown,
}
