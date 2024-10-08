pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 || len > digits.len() {
        return vec![];
    }

    digits
        .chars()
        .collect::<Vec<_>>()
        .windows(len)
        .map(|window| window.iter().collect())
        .collect()
}
