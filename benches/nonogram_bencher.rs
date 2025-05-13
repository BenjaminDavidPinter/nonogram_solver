use criterion::{Criterion, criterion_group, criterion_main, PlotConfiguration, AxisScale};
use nonogram_solver::nonogram::{Hint, Nonogram};

let plot_config = PlotConfiguration::default()
    .summary_scale(AxisScale::Logarithmic);

pub fn gffte(c: &mut Criterion) {
    let mut board = get_test_nonogram();
    let mut hint = Hint::new(4);

    Nonogram::write_column_hint_to_board(&mut board, &mut hint, 0, 0);

    c.bench_function("gffte", |b| {
        b.iter(|| {
            let _distance = Nonogram::gffte(&board, 0);
        });
    });
}

pub fn guch(c: &mut Criterion) {
    let mut board = get_test_nonogram();

    c.bench_function("get_unsolved_column_hints", |b| {
        b.iter(|| {
            Nonogram::get_unsolved_column_hints(&board, 1); // The function you're benchmarking
        });
    });
}

pub fn gurh(c: &mut Criterion) {
   let mut board = get_test_nonogram();

   c.bench_function("get_unsolved_row_hints", |b| {
      b.iter(|| {
         Nonogram::get_unsolved_row_hints(&board, 1); // The function you're benchmarking
      });
   });
}

pub fn gfr(c: &mut Criterion) {
   let mut board = get_test_nonogram();

   c.bench_function("locate_finished_rows", |b| {
      b.iter(|| {
         Nonogram::locate_finished_rows(&board); // The function you're benchmarking
      });
   });
}

pub fn gfc(c: &mut Criterion) {
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

criterion_group!(benches, gffte, guch, gurh, gfc, gfr);
criterion_main!(benches);
