use numquant::quantizer;

#[test]
fn test_accuracy() {
    test_with_values::<u8>(255, 100000);
    test_with_values::<u32>(511, 10000);
    test_with_values::<u32>(100000, 10000);
    test_with_values::<u32>(511, 10000);
}

/// For several values, ensures quantizing and unquantizing it gives back the original value within an allowed difference.
/// Also ensures that the quantized values are evenly distributed across the values from 0 to `q_max`.
fn test_with_values<T>(q_max: T, iterations: usize)
where
    T: Copy + Into<f64> + TryFrom<u64> + std::fmt::Display,
    usize: TryFrom<T>,
    <usize as TryFrom<T>>::Error: std::fmt::Debug,
{
    let epsilon: f64 = quantizer::max_error(q_max.into());

    println!("Testing q_max={q_max}: epsilon={epsilon}");

    let mut histogram = vec![0i32; usize::try_from(q_max).unwrap() + 1];
    let mut min_error: f64 = 0.0;
    let mut max_error: f64 = 0.0;
    for original in (0..=iterations).map(|i| i as f64 / iterations as f64) {
        let quantized = quantizer::quantize(original, q_max);
        histogram[usize::try_from(quantized).unwrap()] += 1;
        let dequantized = quantizer::dequantize(quantized, q_max);
        let error = dequantized - original;
        max_error = max_error.max(error);
        min_error = min_error.min(error);
        println!("{original} => {quantized} => {dequantized}");
        if approx::abs_diff_ne!(dequantized, original, epsilon = epsilon) {
            panic!("Too large error on q_max={q_max}: {original} => {quantized} => {dequantized}: error={error} > {epsilon}");
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
