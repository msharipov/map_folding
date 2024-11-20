use clap::{arg, command, value_parser, ArgAction, Command};
use map_folding::{cli::print_counts_1d, one_d};

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("count")
                .about("Count the number of possible foldings")
                .arg(
                    arg!(-u --upto "Count map foldings for all n ≤ N")
                        .required(false)
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    arg!([N] "The number of segments in the map")
                        .required(true)
                        .value_parser(value_parser!(usize)),
                ),
        )
        .subcommand_required(true)
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("count") {
        let end = *matches.get_one("N").unwrap();
        let start = if matches.get_flag("upto") { 0 } else { end };
        let method = one_d::counting::CountingMethod1d::BruteForce;
        print_counts_1d(start, end, method);
    }
}
