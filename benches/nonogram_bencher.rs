use criterion::{Criterion, criterion_group, criterion_main};
use nonogram_solver::nonogram::{Hint, Nonogram};

/*
        1
     3  1  1
  2 [x][x][ ]
1 1 [x][ ][x]
  2 [x][x][ ]
*/

pub fn get_unsolved_column_hints(c: &mut Criterion) {
    let mut board = get_test_nonogram();

    c.bench_function("get_unsolved_column_hints", |b| {
        b.iter(|| {
            Nonogram::get_unsolved_column_hints(&board, 1); // The function you're benchmarking
        });
    });
}

pub fn get_unsolved_row_hints(c: &mut Criterion) {
   let mut board = get_test_nonogram();

   c.bench_function("get_unsolved_row_hints", |b| {
      b.iter(|| {
         Nonogram::get_unsolved_row_hints(&board, 1); // The function you're benchmarking
      });
   });
}

pub fn locate_finished_rows(c: &mut Criterion) {
   let mut board = get_test_nonogram();

   c.bench_function("locate_finished_rows", |b| {
      b.iter(|| {
         Nonogram::locate_finished_rows(&board); // The function you're benchmarking
      });
   });
}

pub fn locate_finished_columns(c: &mut Criterion) {
   let mut board = get_test_nonogram();

   c.bench_function("locate_finished_columns", |b| {
      b.iter(|| {
         Nonogram::locate_finished_columns(&board); // The function you're benchmarking
      });
   });
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

criterion_group!(benches, get_unsolved_column_hints, get_unsolved_row_hints, locate_finished_columns, locate_finished_rows);
criterion_main!(benches);
