use criterion::{black_box, Criterion};

pub fn in_out_expo2(c: &mut Criterion) {
  let mut group = c.benchmark_group("in_out_expo2");

  group
    .confidence_level(0.99)
    .sample_size(1000)
    .sampling_mode(criterion::SamplingMode::Flat)
    .significance_level(0.1);

  let nums = (0..1_000_000)
    .map(|_num| rand::random::<f32>() * 1000.0)
    .collect::<Vec<_>>();

  group.bench_function("eazing", |b| {
    use eazing::exponential::expo2::InOutExpo2;
    use eazing::Curve;

    b.iter(|| {
      let _ = black_box(
        nums
          .iter()
          .map(|num| InOutExpo2.y(*num))
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
            bevy_tween::interpolation::EaseFunction::ExponentialInOut
              .sample(*num)
          })
          .collect::<Vec<_>>(),
      );
    })
  });

  group.bench_function("emath", |b| {
    b.iter(|| {
      let _ = black_box(
        nums
          .iter()
          .map(|num| emath::easing::exponential_in_out(*num))
          .collect::<Vec<_>>(),
      );
    })
  });

  group.finish();
}
