use criterion::{black_box, Criterion};

pub fn cubic_bezier(c: &mut Criterion) {
  let mut group = c.benchmark_group("cubic_bezier");

  group
    .confidence_level(0.99)
    .sample_size(1000)
    .sampling_mode(criterion::SamplingMode::Flat)
    .significance_level(0.1);

  let nums = (0..1_000_000)
    .map(|_num| rand::random::<f32>() * 1000.0)
    .collect::<Vec<_>>();

  group.bench_function("eazing", |b| {
    use eazing::bezier::cubic_bezier::CubicBezier;
    use eazing::Curve;

    b.iter(|| {
      let _ = black_box(
        nums
          .iter()
          .map(|num| CubicBezier::curve(0.17, 0.67, 0.83, 0.67).y(*num))
          .collect::<Vec<_>>(),
      );
    })
  });

  group.finish();
}
