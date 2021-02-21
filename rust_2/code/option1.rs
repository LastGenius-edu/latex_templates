fn divide(num: f64, den: f64) -> Option<f64> {
    if den == 0.0 {
        None
    } else {
        Some(num / den)
    }
}
