use num_traits::AsPrimitive;
use numquant::linear;
use std::ops::Range;

#[test]
fn test_accuracy() {
    test_with_values::<u8>(100.0..150.0, 255, 100000);
    test_with_values::<u32>(-1000.0..1000.0, 511, 10000);
    test_with_values::<u32>(-100000.0..100000.0, 100000, 10000);
    test_with_values::<u32>(0.0..1e20, 511, 10000);
}

/// For several values, ensures quantizing and unquantizing it gives back the original value within an allowed difference.
/// Also ensures that the quantized values are evenly distributed across the values from 0 to `q_max`.
fn test_with_values<T>(range: Range<f64>, q_max: T, iterations: usize)
where
    T: Copy + AsPrimitive<f64> + std::fmt::Display,
    f64: AsPrimitive<T>,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: std::fmt::Debug,
{
    let epsilon: f64 = linear::max_error(range.clone(), q_max.as_());

    println!("Testing range={range:?} q_max={q_max}: epsilon={epsilon}");

    let mut histogram = vec![0i32; usize::try_from(q_max).unwrap() + 1];
    let mut min_error: f64 = 0.0;
    let mut max_error: f64 = 0.0;
    for fraction in (0..=iterations).map(|i| i as f64 / iterations as f64) {
        let original = range.start + (range.end - range.start) * fraction;
        let quantized = linear::quantize(original, range.clone(), q_max);
        histogram[usize::try_from(quantized).unwrap()] += 1;
        let dequantized = linear::dequantize(quantized, range.clone(), q_max);
        let error = dequantized - original;
        max_error = max_error.max(error);
        min_error = min_error.min(error);
        println!("{original} => {quantized} => {dequantized}");
        if approx::abs_diff_ne!(dequantized, original, epsilon = epsilon) {
            panic!("Too large error on range={range:?} q_max={q_max}: {original} => {quantized} => {dequantized}: error={error} > {epsilon}");
        }
    }

    // Ensure the error is spread evenly across positive and negative values.
    // Epsilon here should be proportional to the values, but I don't know by how much.
    if approx::relative_ne!(max_error - min_error.abs(), 0.0, epsilon = epsilon / 100.0) {
        panic!("min_error = {min_error} max_error = {max_error}: error not evenly distributed");
    }

    // Check number of values quantized to each quantized value
    println!("Histogram:");
    for (i, count) in histogram.iter().enumerate() {
        println!("{i:3}: {count}");
    }
    let min_count = histogram.iter().min().unwrap();
    let max_count = histogram.iter().min().unwrap();
    assert!((max_count - min_count) <= 1);
}
