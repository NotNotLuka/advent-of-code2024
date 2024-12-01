use std::io;
use std::env;
mod days;


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: cargo run <day_number>");
        std::process::exit(1);
    }

    let fun: fn() -> Result<(), io::Error>;
    match args[1].as_str() {
        "1" => {fun = days::day1::day_1},
        _ => {
            eprintln!("No function defined for the given day number");
            std::process::exit(1); 
        }
    }

    if let Err(e) = fun(){
        eprintln!("Error {}", e);
    }
    Ok(())
}
