pub fn square(s: u32) -> u64 {
    1u64 << (s - 1)
}

pub fn total() -> u64 {
    ((1u128 << 64) - 1_u128) as u64
}
