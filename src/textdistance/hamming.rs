use super::algorithm::{Algorithm, Result};

pub struct Hamming {}

impl Default for Hamming {
    fn default() -> Self {
        Self {}
    }
}

impl Algorithm for Hamming {
    fn for_iter<C, E>(&self, mut s1: C, mut s2: C) -> Result
    where
        C: Iterator<Item = E>,
        E: Eq,
    {
        let mut result = 0;
        let mut l1 = 0;
        let mut l2 = 0;
        loop {
            match (s1.next(), s2.next()) {
                (Some(c1), Some(c2)) => {
                    l1 += 1;
                    l2 += 1;
                    if c1 != c2 {
                        result += 1;
                    }
                }
                (Some(_), None) => {
                    l1 += 1;
                    result += 1;
                }
                (None, Some(_)) => {
                    l2 += 1;
                    result += 1;
                }
                (None, None) => {
                    break;
                }
            }
        }
        Result {
            abs: result,
            is_distance: true,
            max: l1.max(l2),
            len1: l1,
            len2: l2,
        }
    }
}

pub fn hamming(s1: &str, s2: &str) -> usize {
    let a: Hamming = Default::default();
    a.for_str(s1, s2).dist()
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn function() {
        let f = hamming;
        assert_eq!(f("", ""), 0);
        assert_eq!(f("", "\0"), 1);
        assert_eq!(f("", "abc"), 3);
        assert_eq!(f("abc", ""), 3);
        assert_eq!(f("sitting", "sitting"), 0);
        assert_eq!(f("abcdefg", "hijklmn"), 7);
        assert_eq!(f("karolin", "kathrin"), 3);
        assert_eq!(f("hello", "world"), 4);
        assert_eq!(f("Rust", "rust"), 1);
        assert_eq!(f("hi mark", "hi markus"), 2);
    }

    #[test]
    fn default_struct_result() {
        let a: Hamming = Default::default();
        let r = a.for_iter("Rust".chars(), "rust".chars());
        assert_eq!(r.dist(), 1);
        assert_eq!(r.sim(), 3);
        assert_eq!(r.max, 4);
        assert_eq!(r.ndist(), 0.25);
    }

    proptest! {
        #[test]
        fn prop(s1 in ".*", s2 in ".*") {
            let res = hamming(&s1, &s2);
            let res2 = hamming(&s2, &s1);
            prop_assert_eq!(res, res2);
            prop_assert!(res <= s1.len() || res <= s2.len());
        }
    }
}
