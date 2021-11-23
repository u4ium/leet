/// A sieve of Eratosthenes
pub struct Sieve {
    /// For each index i,
    ///     if i is prime, factors[i] == i
    ///     else factors[i] == the smallest prime factor of i
    factors: Vec<usize>,
}

impl Sieve {
    /// Create a new sieve of Eratosthenes for primes up to given size (inclusive)
    pub fn new(size: usize) -> Self {
        let mut factors: Vec<_> = (0..=size).collect();
        for i in (2..).take_while(|i| i * i <= size) {
            if factors[i] == i {
                // i is a prime, so all multiples of it are composite
                for j in (i * i..=size).step_by(i) {
                    if factors[j] == j {
                        factors[j] = i;
                    }
                }
            }
        }
        Self { factors }
    }

    /// Iterate over primes factors of n, in descending order
    pub fn prime_factors<'a>(&'a self, n: i32) -> PrimeIterator<'a> {
        debug_assert!(n as usize <= self.factors.len());
        PrimeIterator {
            sieve: &self.factors,
            up_to: n as usize,
        }
    }
}

/// Iterates over primes down to 2, from a given number, in descending order
pub struct PrimeIterator<'a> {
    /// For each index i,
    ///     if i is prime, sieve[i] == i
    ///     else sieve[i] == the smallest prime factor of i
    sieve: &'a Vec<usize>,
    /// The current maximum number to iterate from
    up_to: usize,
}

impl<'a> Iterator for PrimeIterator<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.up_to == 1 {
            return None;
        }
        let prime = self.sieve[self.up_to];
        while self.up_to % prime == 0 {
            self.up_to /= prime;
        }
        Some(prime as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factors_of_8() {
        let sieve = Sieve::new(8);
        let expect = vec![2];
        let actual: Vec<i32> = sieve.prime_factors(8).collect();
        assert_eq!(expect, actual);
    }

    #[test]
    fn factors_of_12() {
        let sieve = Sieve::new(12);
        let expect = vec![2, 3];
        let actual: Vec<i32> = sieve.prime_factors(12).collect();
        assert_eq!(expect, actual);
    }

    #[test]
    fn factors_of_15() {
        let sieve = Sieve::new(15);
        let expect = vec![3, 5];
        let actual: Vec<i32> = sieve.prime_factors(15).collect();
        assert_eq!(expect, actual);
    }

    #[test]
    fn factors_of_39() {
        let sieve = Sieve::new(39);
        let expect = vec![3, 13];
        let actual: Vec<i32> = sieve.prime_factors(39).collect();
        assert_eq!(expect, actual);
    }
}
