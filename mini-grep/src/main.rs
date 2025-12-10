use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigrep::search;



fn main() {
    let args: Vec<String> = env::args().collect();

    // dbg!(args);

    // let query = &args[1];
    // let file_path = &args[2];

    // let config = parse_config(&args);

    // let config = Config::new(&args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });


    // println!("{query}");
    // println!("{}", file_path);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path); 

   // let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    // println!("Contains:\n {contents}");

   // run(config);
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

}

struct Config {
    query: String,
    file_path: String,
}

// fn parse_config(args: &[String]) -> Config {
//    let query = args[1].clone();
//    let file_path = args[2].clone();

//    Config { query, file_path }
// }

impl Config  {
    fn build(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path})
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    for line in search(&config.query, &contents) {
        println!("{line}");
    }

   // println!("With text: \n {contents}");
    Ok(())
}

