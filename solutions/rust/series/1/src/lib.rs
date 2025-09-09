pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut result = Vec::new();

    if len == 0 || digits.len() < len {
        return result;
    }

    for i in 0..=digits.len() - len {
        result.push(digits[i..i + len].to_string());
    }

    result
}
