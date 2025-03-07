use criterion::{black_box, Criterion};

pub fn in_out_quadratic(c: &mut Criterion) {
  let mut group = c.benchmark_group("in_out_quadratic");

  group
    .confidence_level(0.99)
    .sample_size(1000)
    .sampling_mode(criterion::SamplingMode::Flat)
    .significance_level(0.1);

  let nums = (0..1_000_000)
    .map(|_num| rand::random::<f32>() * 1000.0)
    .collect::<Vec<_>>();

  group.bench_function("eazing", |b| {
    use eazing::polynomial::quadratic::InOutQuadratic;
    use eazing::Curve;

    b.iter(|| {
      let _ = black_box(
        nums
          .iter()
          .map(|num| InOutQuadratic.y(*num))
          .collect::<Vec<_>>(),
      );
    })
  });

  group.bench_function("bevy_tween", |b| {
    b.iter(|| {
      let _ = black_box(
        nums
          .iter()
          .map(|num| {
            bevy_tween::interpolation::EaseFunction::QuadraticInOut.sample(*num)
          })
          .collect::<Vec<_>>(),
      );
    })
  });

  group.bench_function("easings", |b| {
    b.iter(|| {
      let _ = black_box(
        nums
          .iter()
          .map(|num| easings::quadratic_in_out(*num as f64))
          .collect::<Vec<_>>(),
      );
    })
  });

  group.bench_function("emath", |b| {
    b.iter(|| {
      let _ = black_box(
        nums
          .iter()
          .map(|num: &f32| emath::easing::quadratic_in_out(*num))
          .collect::<Vec<_>>(),
      );
    })
  });

  group.bench_function("interpolation", |b| {
    use interpolation::Ease;

    b.iter(|| {
      let _ = black_box(
        nums
          .iter()
          .map(|num| num.quadratic_in_out())
          .collect::<Vec<_>>(),
      );
    })
  });

  group.bench_function("simple_easing2", |b| {
    b.iter(|| {
      let _ = black_box(
        nums
          .iter()
          .map(|num| simple_easing2::quad_in_out(*num))
          .collect::<Vec<_>>(),
      );
    })
  });

  group.finish();
}
