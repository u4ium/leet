/// A sieve of Eratosthenes
pub struct Sieve {
    /// For each index i,
    ///  - if i is prime, factors[i] == i
    ///  - else factors[i] == the smallest prime factor of i
    factors: Vec<usize>,
}

impl Sieve {
    /// Create a new sieve of Eratosthenes for primes up to given size (inclusive)
    pub fn new(size: usize) -> Self {
        let mut factors: Vec<_> = (0..=size).collect();
        for i in (2..).take_while(|i| i * i <= size) {
            if factors[i] == i {
                // i is a prime, so all multiples of it are composite
                // also, i*k for any k<i is already marked as a composite of k
                // so only i*(i+k) for k>=0 remains to be marked as a composite of i
                for j in (i * i..=size).step_by(i) {
                    if factors[j] == j {
                        factors[j] = i;
                    }
                }
            }
        }
        Self { factors }
    }

    /// Iterate over primes factors of given composite, starting at 2
    pub fn prime_factors<'a>(&'a self, composite: usize) -> PrimeFactorIterator<'a> {
        debug_assert!(composite <= self.factors.len());
        PrimeFactorIterator {
            sieve: self,
            composite,
        }
    }

    #[inline]
    pub fn is_prime(&self, n: usize) -> bool {
        debug_assert!(n <= self.factors.len());
        self.factors[n] == n
    }
}

/// Iterates over primes factors of a number (from 2, in ascending order)
pub struct PrimeFactorIterator<'a> {
    /// A sieve of Eratosthenes up to (at least) the starting value of composite
    sieve: &'a Sieve,
    /// The (remaining) number to find prime factors of
    composite: usize,
}

impl<'a> Iterator for PrimeFactorIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.composite == 1 {
            return None; // There are no prime factors of 1
        }

        // Find the lowest prime factor of the remaining composite
        let prime = self.sieve.factors[self.composite];

        // Remove this prime factor from composite
        while self.composite % prime == 0 {
            self.composite /= prime;
        }

        // Return this prime factor
        Some(prime)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factors_of_8() {
        let sieve = Sieve::new(8);
        let expect = vec![2];
        let actual: Vec<usize> = sieve.prime_factors(8).collect();
        assert_eq!(expect, actual);
    }

    #[test]
    fn factors_of_12() {
        let sieve = Sieve::new(12);
        let expect = vec![2, 3];
        let actual: Vec<usize> = sieve.prime_factors(12).collect();
        assert_eq!(expect, actual);
    }

    #[test]
    fn factors_of_15() {
        let sieve = Sieve::new(15);
        let expect = vec![3, 5];
        let actual: Vec<usize> = sieve.prime_factors(15).collect();
        assert_eq!(expect, actual);
    }

    #[test]
    fn factors_of_39() {
        let sieve = Sieve::new(39);
        let expect = vec![3, 13];
        let actual: Vec<usize> = sieve.prime_factors(39).collect();
        assert_eq!(expect, actual);
    }
}
