use clap::{Arg, Command};
use std::env::args;

fn std_args() {
    let line_args: Vec<String> = args().collect();

    println!("len: {}, {:?}", line_args.len(), line_args);

    let res = line_args.get(1).unwrap().parse::<i32>();
    match res {
        Ok(num) => println!("Parameter 1 is an integer: {num}"),
        Err(e) => println!("Parameter 1 is not an integer.{:?}", e),
    };
}

fn clap_command() {
    let matches = Command::new("test_args")
        .version("0.1.0")
        .author("Rust learner")
        .about("Test command line arguments")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Input file")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output file")
                .required(true),
        )
        .get_matches();

    let output = matches.get_one::<String>("output").unwrap();
    let input = matches.get_one::<String>("input").unwrap();

    println!("Input file: {}", input);
    println!("Output file: {}", output);
}

fn main() {
    // std_args();
    clap_command();
}
