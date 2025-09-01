const MAX_SIZE: u32 = 64;

pub fn square(s: u32) -> u64 {
    2u64.pow(s - 1)
        .try_into()
        .expect("square must be between 1 and 64")
}

pub fn total() -> u64 {
    (1..=MAX_SIZE).map(|s| square(s)).sum()
}
