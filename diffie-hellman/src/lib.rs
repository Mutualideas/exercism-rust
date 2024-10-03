use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_exponentiation(g as u128, a as u128, p as u128)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_exponentiation(b_pub as u128, a as u128, p as u128)
}

fn modular_exponentiation(mut base: u128, mut exponent: u128, modulus: u128) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result: u128 = 1;
    base %= modulus;

    while exponent > 0 {
        // Check for LSB
        if exponent % 2 == 1 {
            // Multiply result by base if LSB is 1
            result = result * base % modulus;
        }
        // Binary shift of exponent & square base
        exponent >>= 1;
        base = base * base % modulus;
    }
    result as u64
}
