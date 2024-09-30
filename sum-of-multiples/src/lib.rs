pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = Vec::new();
    for &i in factors {
        if i == 0 {
            continue;
        }

        let mut factor = i;
        while factor < limit {
            if !multiples.contains(&factor) {
                multiples.push(factor);
            }
            factor += i;
        }
    }
    multiples.iter().sum()
}
