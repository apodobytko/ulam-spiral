
pub fn generate_primes(max: u64) -> Vec<u64> {
    /// Slightly optimized Sieve of Eratosthenes.
    let mut candidates: Vec<u64> = vec![0; max as usize + 1];
    candidates[0] = 1;
    candidates[1] = 1;
    let max_sqrt = (max as f64).sqrt() as u64;

    for number in 2..max_sqrt {
        let mut multiplied = number * number;
        while multiplied < max {
            candidates[multiplied as usize] = 1;
            multiplied += number;
        }
    }
    candidates
}
