use minigrep::search;
use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    println!("--------------");

    if let Err(err) = run(config) {
        println!("Run error: {err}");
        process::exit(1);
    }
}

struct Config<'a> {
    query: &'a str,
    file_path: &'a str,
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

        Ok(Config { query, file_path })
    }
}

// dyn = dynamic dispatch. aka any implementation of error
// box -> since dyn error is unknown size at compile time
// -> box put it on the heap and give us a pointer
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(config.query, contents) {
        println!("{line}");
    }

    Ok(())
}
