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
   todo!("For later implementation")
}

criterion_group!(benches, benchmark_solve);
criterion_main!(benches);
