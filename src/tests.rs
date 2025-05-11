#[cfg(test)]
mod navigation_tests {
    use crate::nonogram::*;
    
    #[test]
    pub fn locate_finished_rows() {
        let board = get_test_nonogram();
        let finished_rows = Nonogram::locate_finished_rows(&board);

        assert_eq!(finished_rows.len(), 1);
        assert_eq!(finished_rows[0], 7);
    }

    #[test]
    pub fn get_first_filled_squre_from_top_edge() 
    {
        let mut board = get_test_nonogram();
        let mut test_hint = Hint::new(4);
        Nonogram::write_column_hint_to_board(&mut board, &mut test_hint, 1, 3);
        let distance = Nonogram::get_first_filled_square_from_top_edge(&board, 1);
        match distance {
            None => assert_eq!(true, false),
            Some(val) => assert_eq!(val, 3)
        }
    }

    #[test]
    pub fn gffbe_none() 
    {
        let board = get_test_nonogram();
        let distance = Nonogram::gffbe(&board, 1);
        match distance {
            None => assert_eq!(true, true),
            Some(_) => assert_eq!(true, false)
        }
    }

    #[test]
    pub fn gffbe_mid()
    {
        let mut board = get_test_nonogram();
        let mut test_hint = Hint::new(4);
        Nonogram::write_column_hint_to_board(&mut board, &mut test_hint, 1, 3);
        let distance = Nonogram::gffbe(&board, 1);
        match distance {
            None => assert_eq!(true, false),
            Some(val) => assert_eq!(val, 6)
        }
    }

    #[test]
    pub fn gffbe_edge() 
    {
        let mut board = get_test_nonogram(); 
        let mut test_hint = Hint::new(1);
        Nonogram::write_column_hint_to_board(&mut board, &mut test_hint, 1, 0);
        let distance = Nonogram::gffbe(&board, 1);
        match distance {
            None => assert_eq!(true, false),
            Some(val) => assert_eq!(val, 0),
        }
    }

    #[test]
    pub fn get_first_filled_square_from_left_edge() {
        let mut board = get_test_nonogram();
        let mut test_hint = Hint::new(4);
        Nonogram::write_row_hint_to_board(&mut board, &mut test_hint, 1, 3);
        let distance = Nonogram::get_first_filled_square_from_left_edge(&board, 1);
        match distance {
            None => assert_eq!(true, false),
            Some(val) => assert_eq!(val, 3)
        };
    }

    #[test]
    pub fn get_first_filled_square_from_right_edge() {
        let mut board = get_test_nonogram();
        let mut test_hint = Hint::new(4);
        Nonogram::write_row_hint_to_board(&mut board, &mut test_hint, 1, 3);
        let distance = Nonogram::get_first_filled_square_from_right_edge(&board, 1);
        match distance {
            None => assert_eq!(true, false),
            Some(val) => assert_eq!(val, 7)
        }
    }

    #[test]
    pub fn locate_finished_columns() {
        let board = get_test_nonogram();
        let finished_columns = Nonogram::locate_finished_columns(&board);

        assert_eq!(finished_columns.len(), 2);
        assert_eq!(finished_columns[0], 8);
        assert_eq!(finished_columns[1], 9);
    }

    #[test]
    pub fn write_row_hint_to_board() {
        let mut board = get_test_nonogram();
        let mut test_hint = Hint::new(4);
        Nonogram::write_row_hint_to_board(&mut board, &mut test_hint, 1, 3);

        assert_eq!(board.board[1][3], SpaceStatus::O);
        assert_eq!(board.board[1][4], SpaceStatus::O);
        assert_eq!(board.board[1][5], SpaceStatus::O);
        assert_eq!(board.board[1][6], SpaceStatus::O);
        assert_eq!(test_hint.fulfilled, true);
    }

    #[test]
    pub fn write_column_hint_to_board() {
        let mut board = get_test_nonogram();
        let mut test_hint = Hint::new(6);
        Nonogram::write_column_hint_to_board(&mut board, &mut test_hint, 4, 2);

        assert_eq!(board.board[2][4], SpaceStatus::O);
        assert_eq!(board.board[3][4], SpaceStatus::O);
        assert_eq!(board.board[4][4], SpaceStatus::O);
        assert_eq!(board.board[5][4], SpaceStatus::O);
        assert_eq!(board.board[6][4], SpaceStatus::O);
        assert_eq!(board.board[7][4], SpaceStatus::O);

        assert_eq!(test_hint.fulfilled, true);
    }

    #[test]
    pub fn get_unsolved_row_hints(){
        let board = get_test_nonogram();
        let unsolved_hints = Nonogram::get_unsolved_row_hints(&board, 2);
        assert_eq!(unsolved_hints.len(), 2);
        assert_eq!(unsolved_hints[0].0, 0);
        assert_eq!(unsolved_hints[0].1.hint, 1);
        assert_eq!(unsolved_hints[1].0, 1);
        assert_eq!(unsolved_hints[1].1.hint, 2);
    }

    #[test]
    pub fn get_unsolved_column_hints(){
        let board = get_test_nonogram();
        let unsolved_hints = Nonogram::get_unsolved_column_hints(&board, 4);
        assert_eq!(unsolved_hints.len(), 1);
        assert_eq!(unsolved_hints[0].0, 0);
        assert_eq!(unsolved_hints[0].1.hint, 4);
    }

    pub fn get_test_nonogram() -> Nonogram {
        Nonogram::new(
            vec![
                vec![Hint::new(1), Hint::new(2)],
                vec![Hint::new(1), Hint::new(3)],
                vec![Hint::new(1)],
                vec![Hint::new(3)],
                vec![Hint::new(4)],
                vec![Hint::new(5), Hint::new(2)],
                vec![Hint::new(5), Hint::new(2)],
                vec![Hint::new(1), Hint::new(2), Hint::new(2)],
                vec![Hint::new(1), Hint::new(4), Hint::new(3)],
                vec![Hint::new(4), Hint::new(1), Hint::new(3)],
            ],
            vec![
                vec![Hint::new(2), Hint::new(5)],
                vec![Hint::new(2), Hint::new(5)],
                vec![Hint::new(1), Hint::new(2)],
                vec![Hint::new(1)],
                vec![Hint::new(1), Hint::new(2)],
                vec![Hint::new(6)],
                vec![Hint::new(7)],
                vec![Hint::new(2), Hint::new(4), Hint::new(2)],
                vec![Hint::new(2), Hint::new(1)],
                vec![Hint::new(5)],
            ],
        )
    }
}
