pub mod one_d;
use one_d::counting::CountingMethod1d;

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
