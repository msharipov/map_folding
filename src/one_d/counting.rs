use super::folding::MapFolding;
use itertools::Itertools;
use num::bigint::BigUint;

pub enum CountingMethod1d {
    BruteForce,
    SawadaLi,
}

/// Counts the number of possible 1-d foldings by checking every permutation of
/// segments in the standard stack notation.
///
/// This algorithm is implemented primarily for reference and is not useful in
/// practice due to O(n!) time complexity.
///
/// # Example
/// ```
/// use map_folding::one_d::counting::brute_force;
/// use num::bigint::BigUint;
///
/// assert_eq!(BigUint::from(144u32), brute_force(6));
/// ```
pub fn brute_force(n: usize) -> BigUint {
    if n == 0 {
        return BigUint::ZERO;
    }
    let perms = (1..(n + 1) as u64).permutations(n);
    let mut counter = BigUint::ZERO;
    for p in perms {
        if MapFolding::from_stack(&p).unwrap().is_foldable() {
            counter += 1_u32;
        }
    }
    counter
}

pub fn sawada_li(n: usize) -> BigUint {
    return BigUint::ZERO;
}
