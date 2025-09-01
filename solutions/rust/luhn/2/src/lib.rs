/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let digits = match to_digits(code) {
        Some(d) => d,
        None => return false,
    };
    if digits.len() <= 1 {
        return false;
    }

    let sum = digits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &d)| {
            if i % 2 == 1 {
                let doubled = d * 2;
                if doubled > 9 { doubled - 9 } else { doubled }
            } else {
                d
            }
        })
        .sum::<u32>();

    sum % 10 == 0
}

fn to_digits(code: &str) -> Option<Vec<u32>> {
    code.trim()
        .chars()
        .filter(|&c| c != ' ')
        .map(|c| c.to_digit(10))
        .collect()
}
