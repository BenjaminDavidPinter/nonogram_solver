use criterion::{Criterion, criterion_group, criterion_main};
use nonogram_solver::nonogram::{Hint, Nonogram};

/*
        1
     3  1  1
  2 [x][x][ ]
1 1 [x][ ][x]
  2 [x][x][ ]
*/

pub fn benchmark_solve(c: &mut Criterion) {
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

    board.draw_board_to_console();
}

criterion_group!(benches, benchmark_solve);
criterion_main!(benches);
