pub mod one_d;
use one_d::counting::CountingMethod1d;

/// Counts and prints the number of 1-dimensional foldings for n between `start`
/// and `end` (inclusively).
///
/// `method` determines which algorithm is used for counting.
///
/// # Example
/// ```
/// use map_folding::print_counts_1d;
/// use map_folding::one_d::counting::CountingMethod1d;
///
/// print_counts_1d(3, 9, CountingMethod1d::BruteForce);
/// ```
///  Output:
/// ```
/// 3 : 6
/// 4 : 16
/// 5 : 50
/// 6 : 144
/// 7 : 462
/// 8 : 1392
/// 9 : 4536
/// ```
pub fn print_counts_1d(start: usize, end: usize, method: CountingMethod1d) {
    let width = format!("{}", end).len() + 1;
    let counting_method = match method {
        CountingMethod1d::BruteForce => one_d::counting::brute_force,
        CountingMethod1d::SawadaLi => one_d::counting::sawada_li,
    };
    for i in start..end + 1 {
        println!("{:width$} : {}", i, counting_method(i));
    }
}
