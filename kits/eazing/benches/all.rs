mod bench;

use bench::bezier;
use bench::oscillatory::{bounce, elastic};
use bench::polynomial::{bench_linear, cubic, quadratic, quartic};

use criterion::{criterion_group, criterion_main};

#[rustfmt::skip]
criterion_group!(
  benches,
  // polynomial:linear.
  bench_linear::linear,
  // polynomial:quadratic.
  quadratic::bench_in_quadratic::in_quadratic,
  quadratic::bench_in_out_quadratic::in_out_quadratic,
  quadratic::bench_out_quadratic::out_quadratic,
  // polynomial:quartic.
  quartic::bench_in_quartic::in_quartic,
  quartic::bench_in_out_quartic::in_out_quartic,
  quartic::bench_out_quartic::out_quartic,
  // polynomial:cubic.
  cubic::bench_in_cubic::in_cubic,
  cubic::bench_in_out_cubic::in_out_cubic,
  cubic::bench_out_cubic::out_cubic,

  // oscillatory:bounce.
  bounce::bench_in_bounce::in_bounce,
  bounce::bench_in_out_bounce::in_out_bounce,
  bounce::bench_out_bounce::out_bounce,
  // oscillatory:elastic.
  elastic::bench_in_elastic::in_elastic,
  elastic::bench_in_out_elastic::in_out_elastic,
  elastic::bench_out_elastic::out_elastic,

  // betier:cubic_bezier.
  bezier::cubic_bezier::cubic_bezier,
);

criterion_main!(benches);
