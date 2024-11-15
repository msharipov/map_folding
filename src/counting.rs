use itertools::Itertools;
use map_folding::MapFolding;

pub fn brute_force(n: usize) -> usize {
    if n == 0 {
        return 0
    }
    let perms = (1..(n + 1) as u64).permutations(n);
    let mut counter: usize = 0;
    for p in perms {
        if MapFolding::from_stack(&p).unwrap().is_foldable() {
            counter += 1;
        }
    }
    counter
}
