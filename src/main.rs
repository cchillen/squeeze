use std::env;
use std::process;

use squeeze::run;

const EXPECTED_ARG_NUM: usize = 3;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check for valid number of command line arguments.
    if args.len() != EXPECTED_ARG_NUM {
        eprintln!("usage: squeeze <infile> <outfile>");
        process::exit(2);
    }

    // Run squeeze algorithm.
    if let Err(err) = run(args[1].clone(), args[2].clone()) {
        eprintln!("{}", err);
        process::exit(2);
    }
}

