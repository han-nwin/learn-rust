use minigrep::{search, search_case_insensitive};
use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}"); // print to stderr
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    println!("--------------");

    if let Err(err) = run(config) {
        eprintln!("Run error: {err}"); // print to stderr
        process::exit(1);
    }
}

struct Config<'a> {
    query: &'a str,
    file_path: &'a str,
    ignore_case: bool,
}

impl<'a> Config<'a> {
    //the returned Config contains string
    // slices borrowed from args, and it cannot outlive
    // args.
    fn build(args: &'a [String]) -> Result<Config<'a>, &'static str> {
        if args.len() < 3 {
            return Err("Usage: minigrep <query> <file>");
        }
        let query = &args[1];
        let file_path = &args[2];
        let ignore_case = env::var("IGNORE_CASE").is_ok(); // Check if this env var is set

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

// dyn = dynamic dispatch. aka any implementation of error
// box -> since dyn error is unknown size at compile time
// -> box put it on the heap and give us a pointer
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(config.query, &contents)
    } else {
        search(config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
