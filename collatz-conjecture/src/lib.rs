pub fn collatz(mut n: u64) -> Option<u64> {
    let mut steps: u64 = 0;

    // Handle 0
    if n == 0 {
        return None;
    }

    while n != 1 {
        if n % 2 == 0 {
            n /= 2
        } else {
            n = n * 3 + 1
        }
        steps += 1;
    }
    Some(steps)
}
