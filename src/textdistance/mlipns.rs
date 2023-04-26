use super::algorithm::{Algorithm, Result};
use super::hamming::Hamming;
use std::hash::Hash;

pub struct MLIPNS {
    hamming: Hamming,
    threshold: f64,
    max_mismatches: usize,
}

impl Default for MLIPNS {
    fn default() -> Self {
        Self {
            hamming: Default::default(),
            threshold: 0.25,
            max_mismatches: 2,
        }
    }
}

impl MLIPNS {
    fn check(&self, ham: &Result<usize>) -> bool {
        let mut mismatches = 0;
        let mut max_length = ham.max;
        let mut ham_val = ham.val();
        while mismatches <= self.max_mismatches {
            if max_length == 0 {
                return true;
            }
            if (1.0 - (max_length - ham_val) as f64 / max_length as f64) <= self.threshold {
                return true;
            }
            mismatches += 1;
            ham_val -= 1;
            max_length -= 1;
        }
        max_length == 0
    }
}

impl Algorithm<usize> for MLIPNS {
    fn for_iter<C, E>(&self, s1: C, s2: C) -> Result<usize>
    where
        C: Iterator<Item = E>,
        E: Eq + Copy + Hash,
    {
        let ham = self.hamming.for_iter(s1, s2);
        Result {
            abs: if self.check(&ham) { 1 } else { 0 },
            is_distance: false,
            max: 1,
            len1: ham.len1,
            len2: ham.len2,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::textdistance::str::mlipns;
    use assert2::assert;
    use rstest::rstest;

    #[rstest]
    #[case("", "", 1)]
    fn function_str(#[case] s1: &str, #[case] s2: &str, #[case] exp: usize) {
        assert!(mlipns(s1, s2) == exp);
    }
}
