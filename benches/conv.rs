use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn v1(i: u128) -> u64 {
    let n = i.leading_zeros();
    let y = i.wrapping_shl(n);
    let a = (y >> 75) as u64; // Significant bits, with bit 53 still in tact.
    let b = (y >> 11 | y & 0xFFFF_FFFF) as u64; // Insignificant bits, only relevant for rounding.
    let m = a + ((b - (b >> 63 & !a)) >> 63); // Add one when we need to round up. Break ties to even.

    // Use `if`
    let e = if i == 0 { 0 } else { 1149 - n as u64 }; // Exponent plus 1023, minus one, except for zero.
                                                      //
    (e << 52) + m // + not |, so the mantissa can overflow into the exponent.
}

fn v2(i: u128) -> u64 {
    let n = i.leading_zeros();
    let y = i.wrapping_shl(n);
    let a = (y >> 75) as u64; // Significant bits, with bit 53 still in tact.
    let b = (y >> 11 | y & 0xFFFF_FFFF) as u64; // Insignificant bits, only relevant for rounding.
    let m = a + ((b - (b >> 63 & !a)) >> 63); // Add one when we need to round up. Break ties to even.

    // Use predication instead of `if`
    let e = (i != 0) as u64 * (1149 - n as u64); // Exponent plus 1023, minus one, except for zero.
                                                 //
    (e << 52) + m // + not |, so the mantissa can overflow into the exponent.
}

fn convert_v1(i: u128) -> f64 {
    f64::from_bits(v1(i))
}

fn convert_v2(i: u128) -> f64 {
    f64::from_bits(v2(i))
}

fn criterion_benchmark(c: &mut Criterion) {
    let value = 203109040335017441194027291691239031151;

    let mut group = c.benchmark_group("conv");
    group.bench_function("if", |b| b.iter(|| convert_v1(black_box(value))));
    group.bench_function("pred", |b| b.iter(|| convert_v2(black_box(value))));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
