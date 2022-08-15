pub fn non_empty_str(input: Option<String>) -> Option<String> {
    match &input {
        Some(str) => if str.is_empty() { None } else { Some(str.to_string()) }
        None => None
    }
}