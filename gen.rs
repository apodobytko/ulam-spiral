

fn generate_primes_slow() -> Vec<u64> {
    let candidates: Vec<u64> = [0; 100000].to_vec();
    let cloned_candidates: Vec<u64> = candidates.to_vec();
    let mut again: Vec<u64> = candidates.to_vec();

    for (i, _) in candidates.iter().enumerate().skip(2) {

        for (ii, _) in cloned_candidates
                .iter()
                .enumerate()
                .skip(2)
                .filter(
                    |&(index, _)| (index as u64) < (cloned_candidates.len() as f64).sqrt() as u64
                ) {

            if (i != ii) & (i as u64 % ii as u64 == 0) {
                again[i] = 1;
                break;
            }
        }
        let cloned_candidates = again.to_vec();
    }


    for (i, _) in again.iter().enumerate().filter(|&(_, n)| *n == 0) {
        println!("result {}", i);
    }

    again
}

fn generate_primes(max: u64) -> Vec<u64> {
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


fn main() {
    generate_primes(10_000_000);
}


// filter(|&&n| n < (*number as f64).sqrt() as u64)
// 2  2
// 3 
// 4
// 5