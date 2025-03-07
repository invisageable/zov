use criterion::{black_box, Criterion};

pub fn linear(c: &mut Criterion) {
  let mut group = c.benchmark_group("linear");

  group
    .confidence_level(0.99)
    .sample_size(1000)
    .sampling_mode(criterion::SamplingMode::Flat)
    .significance_level(0.1);

  let nums = (0..1_000_000)
    .map(|_num| rand::random::<f32>() * 1000.0)
    .collect::<Vec<_>>();

  group.bench_function("eazing", |b| {
    use eazing::polynomial::linear::Linear;
    use eazing::Curve;

    b.iter(|| {
      let _ =
        black_box(nums.iter().map(|num| Linear.y(*num)).collect::<Vec<_>>());
    })
  });

  group.bench_function("bevy_tween", |b| {
    b.iter(|| {
      let _ = black_box(
        nums
          .iter()
          .map(|num| {
            bevy_tween::interpolation::EaseFunction::Linear.sample(*num)
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
          .map(|num| easings::linear(*num as f64))
          .collect::<Vec<_>>(),
      );
    })
  });

  group.bench_function("emath", |b| {
    b.iter(|| {
      let _ = black_box(
        nums
          .iter()
          .map(|num: &f32| emath::easing::linear(*num))
          .collect::<Vec<_>>(),
      );
    })
  });

  group.bench_function("glissade", |b| {
    b.iter(|| {
      let _ = black_box(
        nums
          .iter()
          .map(|num: &f32| glissade::Easing::default().ease(*num))
          .collect::<Vec<_>>(),
      );
    })
  });

  group.bench_function("keyframe", |b| {
    use keyframe::EasingFunction;

    b.iter(|| {
      let _ = black_box(
        nums
          .iter()
          .map(|num| keyframe::functions::Linear::default().y(*num as f64))
          .collect::<Vec<_>>(),
      );
    })
  });

  group.bench_function("simple_easing2", |b| {
    b.iter(|| {
      let _ = black_box(
        nums
          .iter()
          .map(|num| simple_easing2::linear(*num))
          .collect::<Vec<_>>(),
      );
    })
  });

  group.finish();
}
