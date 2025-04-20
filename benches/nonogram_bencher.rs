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
            vec![Hint::new(3)],
            vec![Hint::new(1), Hint::new(1)],
            vec![Hint::new(1)],
        ],
        vec![
            vec![Hint::new(2)],
            vec![Hint::new(1), Hint::new(1)],
            vec![Hint::new(2)],
        ],
    );

    board.set_square(0, 0, true);
    board.set_square(0, 1, true);

    board.set_square(1, 0, true);
    board.set_square(1, 2, true);

    board.set_square(2, 0, true);
    board.set_square(2, 1, true);

    board.draw_board_to_console();
}

criterion_group!(benches, benchmark_solve);
criterion_main!(benches);
