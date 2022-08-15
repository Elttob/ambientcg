pub fn non_zero_f32(input: Option<f32>) -> Option<f32> {
    match &input {
        Some(num) => if *num == 0.0 { None } else { Some(*num) }
        None => None
    }
}