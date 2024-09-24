use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn to_bits_if(i: u128) -> u64 {
    let n = i.leading_zeros();
    let y = i.wrapping_shl(n);
    let a = (y >> 75) as u64; // Significant bits, with bit 53 still in tact.
    let b = (y >> 11 | y & 0xFFFF_FFFF) as u64; // Insignificant bits, only relevant for rounding.
    let m = a + ((b - (b >> 63 & !a)) >> 63); // Add one when we need to round up. Break ties to even.

    // Use `if`
    let e = if i == 0 { 0 } else { 1149 - n as u64 }; // Exponent plus 1023, minus one, except for zero.

    (e << 52) + m // + not |, so the mantissa can overflow into the exponent.
}

fn to_bits_pred(i: u128) -> u64 {
    let n = i.leading_zeros();
    let y = i.wrapping_shl(n);
    let a = (y >> 75) as u64; // Significant bits, with bit 53 still in tact.
    let b = (y >> 11 | y & 0xFFFF_FFFF) as u64; // Insignificant bits, only relevant for rounding.
    let m = a + ((b - (b >> 63 & !a)) >> 63); // Add one when we need to round up. Break ties to even.

    // Use predication instead of `if`
    let e = ((i != 0) as u64) * ((1149 - n) as u64); // Exponent plus 1023, minus one, except for zero.

    (e << 52) + m // + not |, so the mantissa can overflow into the exponent.
}

fn convert_with_if(i: u128) -> f64 {
    f64::from_bits(to_bits_if(i))
}

fn convert_with_pred(i: u128) -> f64 {
    f64::from_bits(to_bits_pred(i))
}

fn criterion_benchmark(c: &mut Criterion) {
    let value = 203109040335017441194027291691239031151;

    let mut group = c.benchmark_group("conv");
    group.bench_function("if", |b| b.iter(|| convert_with_if(black_box(value))));
    group.bench_function("pred", |b| b.iter(|| convert_with_pred(black_box(value))));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
