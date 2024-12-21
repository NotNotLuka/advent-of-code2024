use std::io;
use std::env;
use std::time::Instant;
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
        "2" => {fun = days::day2::day_2},
        "3" => {fun = days::day3::day_3},
        "4" => {fun = days::day4::day_4},
        "5" => {fun = days::day5::day_5},
        "6" => {fun = days::day6::day_6},
        "7" => {fun = days::day7::day_7},
        "8" => {fun = days::day8::day_8},
        "9" => {fun = days::day9::day_9},
        "10" => {fun = days::day10::day_10},
        "11" => {fun = days::day11::day_11},
        "12" => {fun = days::day12::day_12},
        "13" => {fun = days::day13::day_13},
        "14" => {fun = days::day14::day_14},
        "15" => {fun = days::day15::day_15},
        "16" => {fun = days::day16::day_16},
        "17" => {fun = days::day17::day_17},
        "18" => {fun = days::day18::day_18},
        "19" => {fun = days::day19::day_19},
        "20" => {fun = days::day20::day_20},
        _ => {
            eprintln!("No function defined for the given day number");
            std::process::exit(1); 
        }
    }

    let now = Instant::now();
    if let Err(e) = fun(){
        eprintln!("Error {}", e);
    }
    let time = now.elapsed().as_micros() as f64/1000.;
    println!("Time taken: {}ms", time);
    Ok(())
}

