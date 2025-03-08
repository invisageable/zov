use data::bench::{
  CONFIDENCE_LEVEL, NUMS_TOTAL, SAMPLE_SIZE, SIGNIFICANCE_LEVEL,
};

use criterion::{black_box, Criterion};

pub fn in_log10(c: &mut Criterion) {
  let mut group = c.benchmark_group("in_log10");

  group
    .confidence_level(CONFIDENCE_LEVEL)
    .sample_size(SAMPLE_SIZE)
    .sampling_mode(criterion::SamplingMode::Flat)
    .significance_level(SIGNIFICANCE_LEVEL);

  let nums = (0..NUMS_TOTAL)
    .map(|_num| rand::random::<f32>() * 1000.0)
    .collect::<Vec<_>>();

  group.bench_function("eazing", |b| {
    use eazing::logarithmic::log10::InLog10;
    use eazing::Curve;

    b.iter(|| {
      let _ =
        black_box(nums.iter().map(|num| InLog10.y(*num)).collect::<Vec<_>>());
    })
  });

  group.finish();
}
