pub fn collatz(n: u64) -> Option<u64> {
    let mut count = 0;
    let mut current = n;

    if n == 0 {
        return None;
    }

    while current != 1 {
        if current % 2 == 0 {
            current /= 2;
        } else {
            current = 3 * current + 1;
        }

        count += 1;
    }

    Some(count)
}
