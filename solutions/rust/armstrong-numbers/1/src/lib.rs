pub fn is_armstrong_number(num: u32) -> bool {
    let code = num.to_string();
    let len = code.len() as u32;

    code.chars()
        .map(|c| c.to_digit(10).unwrap().pow(len))
        .sum::<u32>()
        == num
}
