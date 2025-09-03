pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }

    let mut count = 0;
    let mut candidate = 1;

    while count < n {
        candidate += 2; // 奇数のみチェック
        if is_prime(candidate) {
            count += 1;
        }
    }

    candidate
}

fn is_prime(num: u32) -> bool {
    if num < 2 {
        return false;
    }
    if num == 2 {
        return true;
    }
    if num % 2 == 0 {
        return false;
    }

    let limit = (num as f64).sqrt() as u32;
    for i in (3..=limit).step_by(2) {
        if num % i == 0 {
            return false;
        }
    }
    true
}
