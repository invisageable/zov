use criterion::{black_box, Criterion};

pub fn in_out_cubic(c: &mut Criterion) {
  let mut group = c.benchmark_group("in_out_cubic");

  group
    .confidence_level(0.99)
    .sample_size(1000)
    .sampling_mode(criterion::SamplingMode::Flat)
    .significance_level(0.1);

  let nums = (0..1_000_000)
    .map(|_num| rand::random::<f32>() * 1000.0)
    .collect::<Vec<_>>();

  group.bench_function("eazing", |b| {
    use eazing::polynomial::quartic::InQuartic;
    use eazing::Curve;

    b.iter(|| {
      let _ =
        black_box(nums.iter().map(|num| InQuartic.y(*num)).collect::<Vec<_>>());
    })
  });

  group.bench_function("bevy_tween", |b| {
    b.iter(|| {
      let _ = black_box(
        nums
          .iter()
          .map(|num| {
            bevy_tween::interpolation::EaseFunction::QuarticIn.sample(*num)
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
          .map(|num| easings::quartic_in(*num as f64))
          .collect::<Vec<_>>(),
      );
    })
  });

  group.bench_function("emath", |b| {
    b.iter(|| {
      let _ = black_box(
        nums
          .iter()
          .map(|num| emath::easing::cubic_in_out(*num))
          .collect::<Vec<_>>(),
      );
    })
  });

  group.bench_function("glissade", |b| {
    b.iter(|| {
      let _ = black_box(
        nums
          .iter()
          .map(|num| glissade::Easing::QuarticIn.ease(*num))
          .collect::<Vec<_>>(),
      );
    })
  });

  group.bench_function("interpolation", |b| {
    use interpolation::Ease;

    b.iter(|| {
      let _ =
        black_box(nums.iter().map(|num| num.quartic_in()).collect::<Vec<_>>());
    })
  });

  group.bench_function("keyframe", |b| {
    use keyframe::EasingFunction;

    let quartic = keyframe::functions::EaseInQuart::default();

    b.iter(|| {
      let _ = black_box(
        nums
          .iter()
          .map(|num| quartic.y(*num as f64))
          .collect::<Vec<_>>(),
      );
    })
  });

  group.bench_function("simple_easing2", |b| {
    b.iter(|| {
      let _ = black_box(
        nums
          .iter()
          .map(|num| simple_easing2::quart_in(*num))
          .collect::<Vec<_>>(),
      );
    })
  });

  group.finish();
}
