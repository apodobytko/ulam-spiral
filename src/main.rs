
mod sieve;
mod spiral;

use self::sieve::generate_primes;
use self::spiral::generate_spiral;



fn main() {
    let primes = generate_primes(1_000_000);
    generate_spiral(primes);
}
