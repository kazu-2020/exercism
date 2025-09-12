pub fn abbreviate(phrase: &str) -> String {
    phrase
        .chars()
        .enumerate()
        .filter_map(|(i, c)| {
            if !c.is_alphabetic() {
                return None;
            }

            let is_start = i == 0 || {
                let prev_char = phrase.chars().nth(i - 1).unwrap();
                !prev_char.is_alphabetic() || prev_char.is_lowercase() && c.is_uppercase()
            };

            if is_start {
                Some(c.to_ascii_uppercase())
            } else {
                None
            }
        })
        .collect()
}
