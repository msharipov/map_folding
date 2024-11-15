mod counting;
use clap::{arg, command, value_parser, ArgAction};

fn main() {
    let matches = command!()
        .arg(
            arg!(-u --upto "Count map foldings for all n < N")
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .arg(
            arg!([N] "The number of segments in the map")
                .required(true)
                .value_parser(value_parser!(usize)),
        )
        .get_matches();

    let n_max: usize = *matches.get_one("N").unwrap();
    let n_min = if matches.get_flag("upto") { 1 } else { n_max };

    let width = format!("{}", n_max).len() + 1;
    for i in n_min..n_max + 1 {
        println!("{:width$} : {}", i, counting::brute_force(i));
    }
}
