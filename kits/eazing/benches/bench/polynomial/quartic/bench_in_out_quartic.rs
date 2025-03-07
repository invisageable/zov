use criterion::{black_box, Criterion};

pub fn in_out_quartic(c: &mut Criterion) {
  let mut group = c.benchmark_group("in_out_quartic");

  group
    .confidence_level(0.99)
    .sample_size(1000)
    .sampling_mode(criterion::SamplingMode::Flat)
    .significance_level(0.1);

  let nums = (0..1_000_000)
    .map(|_num| rand::random::<f32>() * 1000.0)
    .collect::<Vec<_>>();

  group.bench_function("eazing", |b| {
    use eazing::polynomial::quartic::InOutQuartic;
    use eazing::Curve;

    b.iter(|| {
      let _ = black_box(
        nums
          .iter()
          .map(|num| InOutQuartic.y(*num))
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
            bevy_tween::interpolation::EaseFunction::QuarticInOut.sample(*num)
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
          .map(|num| easings::quartic_in_out(*num as f64))
          .collect::<Vec<_>>(),
      );
    })
  });

  group.bench_function("glissade", |b| {
    b.iter(|| {
      let _ = black_box(
        nums
          .iter()
          .map(|num| glissade::Easing::QuarticInOut.ease(*num))
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
          .map(|num| num.quartic_in_out())
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
          .map(|num| {
            keyframe::functions::EaseInOutQuart::default().y(*num as f64)
          })
          .collect::<Vec<_>>(),
      );
    })
  });

  group.bench_function("simple_easing2", |b| {
    b.iter(|| {
      let _ = black_box(
        nums
          .iter()
          .map(|num| simple_easing2::quart_in_out(*num))
          .collect::<Vec<_>>(),
      );
    })
  });

  group.finish();
}
