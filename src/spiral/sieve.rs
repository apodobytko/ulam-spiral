extern crate rand;

pub fn generate_primes(max: u64) -> Vec<u64> {
    // Slightly optimized Sieve of Eratosthenes.
    let mut candidates: Vec<u64> = vec![1; max as usize + 1];
    candidates[0] = 0;
    candidates[1] = 0;
    let max_sqrt = (max as f64).sqrt() as u64 + 1;

    for number in 2..max_sqrt {
        let mut multiplied = number * number;
        while multiplied < max + 1 {
            candidates[multiplied as usize] = 0;
            multiplied += number;
        }
    }
    candidates
}

pub fn generate_random(max: u64) -> Vec<u64> {
    (0..max).map(|n| if n % 2 > 0 { rand::random::<bool>() as u64 } else { 0 })
            .collect::<Vec<u64>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_primes() {
        let result_vec = generate_primes(20);
        assert_eq!(
            result_vec,
            vec![0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0]
            //   0  1  2  3  4  5  6  7  8  9  10 11 12 13 14 15 16 17 18 19 20
        );
    }
}