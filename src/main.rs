mod sieve;

use self::sieve::generate_primes;

fn main() {
    generate_primes(10000);
}
