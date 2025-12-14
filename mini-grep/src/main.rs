// use minigrep::search;
use std::env;
use std::process;

use minigrep::Config;
// use minigrep::search;

fn main() {
    let args: Vec<String> = env::args().collect();

    // dbg!(args);

    // let query = &args[1];
    // let file_path = &args[2];

    // let config = parse_config(&args);

    // let config = Config::new(&args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // println!("{query}");
    // println!("{}", file_path);

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    // let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    // println!("Contains:\n {contents}");

    // run(config);
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

// fn parse_config(args: &[String]) -> Config {
//    let query = args[1].clone();
//    let file_path = args[2].clone();

//    Config { query, file_path }
// }
