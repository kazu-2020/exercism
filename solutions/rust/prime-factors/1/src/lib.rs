pub fn factors(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut result = Vec::new();
    let mut divisor = 2;

    while n > 1 {
        while n % divisor == 0 {
            result.push(divisor);
            n /= divisor;
        }
        divisor += 1;
    }

    result
}
