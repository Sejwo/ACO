pub fn usize_float_multiplication(value: usize, multiplier: f64) -> usize{
    (value as f64*multiplier).round() as usize
}