pub fn nth(n: u32) -> u32 {
    let mut prime_count: u32 = 0; // Track how many primes we've found
    let mut counter: u32 = 1; // Start checking numbers from 2 onwards

    while prime_count <= n {
        counter += 1;
        if is_prime(counter) {
            prime_count += 1;
        }
    }
    counter
}

fn is_prime(num: u32) -> bool {
    for i in 2..=((num as f64).sqrt() as u32) {
        if num % i == 0 {
            return false;
        }
    }
    true
}
