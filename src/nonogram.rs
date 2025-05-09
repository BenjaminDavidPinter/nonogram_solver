use std::{io::Write};
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
        let board = vec![vec![SpaceStatus::Empty; height]; width];
        Nonogram {
            width,
            height,
            board,
            column_hints,
            row_hints,
        }
    }

    pub fn get_board_state(&self) -> [(SpaceStatus, usize); 3] {
        return [
            (
                SpaceStatus::O,
                self.board
                    .iter()
                    .flatten()
                    .filter(|x| **x == SpaceStatus::O)
                    .collect::<Vec<_>>()
                    .len(),
            ),
            (
                SpaceStatus::X,
                self.board
                    .iter()
                    .flatten()
                    .filter(|x| **x == SpaceStatus::X)
                    .collect::<Vec<_>>()
                    .len(),
            ),
            (
                SpaceStatus::Empty,
                self.board
                    .iter()
                    .flatten()
                    .filter(|x| **x == SpaceStatus::Empty)
                    .collect::<Vec<_>>()
                    .len(),
            ),
        ];
    }

    //TODO: Rewrite this as functional code
    pub fn get_first_filled_square_from_bottom_edge(nonogram: &Nonogram, column: usize) -> Option<usize> {
        for square_index in 0..nonogram.height
        {
            let inverted_index = nonogram.height - square_index;
            if nonogram.board[inverted_index - 1][column] == SpaceStatus::O 
            {
                return Some(inverted_index);
            }
        }
        return None;
    }

    //TODO: Rewrite this as functional code
    pub fn get_first_filled_square_from_top_edge(nonogram: &Nonogram, column: usize) -> Option<usize> 
    {
        for square_index in 0..nonogram.height
        {
            if nonogram.board[square_index][column] == SpaceStatus::O {
                return Some(square_index);
            }
        }
        return None;
    }

    //TODO: Rewrite this as functional code
    pub fn get_first_filled_square_from_left_edge(nonogram: &Nonogram, row: usize) -> Option<usize> 
    {
       for square_index in 0..nonogram.board[row].len()
       {
           if nonogram.board[row][square_index] == SpaceStatus::O {
               return Some(square_index);
           }
       }
       return None;
    }

    //TODO: Rewrite this as functional code
    pub fn get_first_filled_square_from_right_edge(nonogram: &Nonogram, row: usize) -> Option<usize> 
    {
        for square_index in 0..nonogram.board[row].len() 
        {
            let inverted_index = nonogram.board[row].len() - square_index;
            if nonogram.board[row][inverted_index-1] == SpaceStatus::O {
                return Some(inverted_index);
            }
        }
        return None;
    }

    pub fn get_unsolved_row_hints(nonogram: &Nonogram, row: usize) -> Vec<(usize, &Hint)>
    {
        nonogram.row_hints[row]
            .iter()
            .enumerate()
            .filter(|(_, x)| !x.fulfilled)
            .collect()
    }

    pub fn get_unsolved_column_hints(nonogram: &Nonogram, column: usize) -> Vec<(usize, &Hint)>
    {
        nonogram.column_hints[column]
            .iter()
            .enumerate()
            .filter(|(_, x)| !x.fulfilled)
            .collect()
    }

    pub fn write_row_hint_to_board(nonogram: &mut Nonogram, hint: &mut Hint, row: usize, starting_index: usize) {
        for column in starting_index..starting_index + hint.hint as usize {
            Nonogram::check_set(nonogram, row, column);
            nonogram.board[row][column] = SpaceStatus::O;
        }
        hint.fulfilled = true;
    }

    pub fn write_column_hint_to_board(nonogram: &mut Nonogram, hint: &mut Hint, column: usize, starting_index: usize) {
        for row in starting_index..starting_index + hint.hint as usize {
            Nonogram::check_set(nonogram, row, column);
            nonogram.board[row][column] = SpaceStatus::O;
        }
        hint.fulfilled = true;
    }

    pub fn locate_finished_rows(nonogram: &Nonogram) -> Vec<usize> {
        nonogram.row_hints
            .iter().enumerate()
            .filter(|(_, hints)| {
                nonogram.width == hints
                    .iter()
                    .map(|x| x.hint as usize)
                    .sum::<usize>() + hints.len() - 1
            })
            .map(|(row_index, _)| row_index)
            .collect()
    }

    pub fn locate_finished_columns(nonogram: &Nonogram) -> Vec<usize> {
        nonogram.column_hints
            .iter().enumerate()
            .filter(|(_, hints)| {
                nonogram.height == hints
                    .iter()
                    .map(|x| x.hint as usize)
                    .sum::<usize>() + hints.len() - 1
            })
            .map(|(row_index, _)| row_index)
            .collect()
    }

    pub fn check_set(nonogram: &Nonogram, row: usize, column: usize)
    {
        if nonogram.board[row][column] != SpaceStatus::Empty {
            panic!("Attempt to set previously set space")
        }
    }

    pub fn draw_board_to_console(nonogram: &Nonogram) {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);

        let max_column_hint_depth = nonogram
            .column_hints
            .iter()
            .map(|column| column.len())
            .max()
            .unwrap_or(0);
        let max_row_hint_depth = nonogram
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
            let mut printable_rows = nonogram
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
        for hint_column in 0..nonogram.row_hints.len() {
            let depth_difference = max_row_hint_depth - nonogram.row_hints[hint_column].len();
            for _ in 0..depth_difference {
                print!("   ");
            }
            for hint in &nonogram.row_hints[hint_column] {
                print!(" {} ", hint.hint)
            }
            for board_space in &nonogram.board[hint_column] {
                match board_space {
                    SpaceStatus::O => {
                        _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)));
                        _ = write!(&mut stdout, "[O]");
                        _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)));
                    }
                    SpaceStatus::X => {
                        _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)));
                        _ = write!(&mut stdout, "[X]");
                        _ = stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)));
                    }
                    SpaceStatus::Empty => {
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

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug, PartialEq)]
pub enum SpaceStatus {
    O,
    X,
    Empty,
}
