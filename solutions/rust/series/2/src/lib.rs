pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut result = Vec::new();

    if len == 0 || digits.len() < len {
        return result;
    }

    digits
        .chars()
        .collect::<Vec<_>>()
        .windows(len)
        .map(|window| window.iter().collect())
        .collect()
}
