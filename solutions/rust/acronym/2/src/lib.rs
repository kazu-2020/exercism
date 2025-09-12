pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(&[' ', '-'])
        .flat_map(|word| {
            let chars: Vec<char> = word.chars().filter(|c| c.is_alphabetic()).collect();
            let mut res = Vec::new();

            for (i, &c) in chars.iter().enumerate() {
                if i == 0 || (c.is_uppercase() && chars[i - 1].is_lowercase()) {
                    res.push(c.to_ascii_uppercase());
                }
            }

            res
        })
        .collect::<String>()
}
