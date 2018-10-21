
pub fn generate_primes(max: u64) -> Vec<u64> {
    /// Slightly optimized Sieve of Eratosthenes.
    let mut candidates: Vec<u64> = vec![0; max as usize + 1];
    candidates[0] = 1;
    candidates[1] = 1;
    let max_sqrt = (max as f64).sqrt() as u64;

    for number in 2..max_sqrt {
        let mut multiplied = number * number;
        while multiplied < max+1 {
            candidates[multiplied as usize] = 1;
            multiplied += number;
        }
    }
    candidates.iter()
              .enumerate()
              .map(|(i, &n)| if n == 0 {i as u64} else {0})
              .filter(|&n| n != 0)
              .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_primes() {
        let result_vec = generate_primes(20);
        assert_eq!(result_vec, vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }

}