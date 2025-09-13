#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    validate_bases(from_base, to_base)?;
    validate_digits(number, from_base)?;

    let trimmed_number = trim_leading_zeros(number);
    if is_zero(trimmed_number) {
        return Ok(vec![0]);
    }

    Ok(from_digit(to_digit(number, from_base), to_base))
}

fn validate_bases(from_base: u32, to_base: u32) -> Result<(), Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }
    Ok(())
}

fn validate_digits(number: &[u32], base: u32) -> Result<(), Error> {
    if let Some(&invalid_digit) = number.iter().find(|&&digit| digit >= base) {
        return Err(Error::InvalidDigit(invalid_digit));
    }
    Ok(())
}

fn trim_leading_zeros(number: &[u32]) -> &[u32] {
    if number.is_empty() {
        return number;
    }

    number
        .iter()
        .position(|&digit| digit != 0)
        .map(|index| &number[index..])
        .unwrap_or(&number[number.len() - 1..])
}

fn is_zero(number: &[u32]) -> bool {
    number.is_empty() || number.iter().all(|&digit| digit == 0)
}

fn to_digit(number: &[u32], base: u32) -> u32 {
    number.iter().fold(0, |acc, &digit| acc * base + digit) // ホーナー法
}

fn from_digit(mut decimal: u32, base: u32) -> Vec<u32> {
    let mut result = Vec::new();
    while decimal > 0 {
        result.push(decimal % base);
        decimal /= base;
    }
    result.reverse();
    result
}
