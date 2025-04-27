use crate::nonogram::{Hint, Nonogram};

#[test]
pub fn test() {
    let mut board = Nonogram::new(
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
    );

    board.solve();
    board.solve_strat_ended_rows();

    println!("----------------------------------");
    println!();
    println!();
    board.draw_board_to_console();
    println!();
    println!("{:?}", board.get_board_state());
    println!();
    println!("-----------------------------------");


    assert_eq!(true, true);
}
