use std::env;

mod bigint_solution;
mod rug_solution;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <solution_type> <parallel> <input_file>", args[0]);
        eprintln!("solution_type: bigint or rug");
        eprintln!("parallel: true or false");
        std::process::exit(1);
    }

    let solution_type = &args[1];
    let parallel = &args[2];
    let input_file = &args[3];

    let use_parallel = match parallel.as_str() {
        "true" => true,
        "false" => false,
        _ => {
            eprintln!("Invalid parallel option. Use 'true' or 'false'");
            std::process::exit(1);
        }
    };

    match solution_type.as_str() {
        "bigint" => bigint_solution::run(input_file, use_parallel),
        "rug" => rug_solution::run(input_file, use_parallel),
        _ => {
            eprintln!("Invalid solution type. Use 'bigint' or 'rug'");
            std::process::exit(1);
        }
    }
}
