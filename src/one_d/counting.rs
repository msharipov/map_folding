use super::folding::MapFolding;
use itertools::Itertools;
use num::bigint::BigUint;

pub enum CountingMethod1d {
    BruteForce,
    SawadaLi,
}

pub fn brute_force(n: usize) -> BigUint {
    if n == 0 {
        return BigUint::ZERO;
    }
    let perms = (1..(n + 1) as u64).permutations(n);
    let mut counter = BigUint::ZERO;
    for p in perms {
        if MapFolding::from_stack(&p).unwrap().is_foldable() {
            counter += 1 as u32;
        }
    }
    counter
}

pub fn sawada_li(n: usize) -> BigUint {
    return BigUint::ZERO;
}
