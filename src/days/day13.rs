use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;


fn regex_numbers(line: String, re: &Regex) -> (i64, i64){
    let captures = re.captures(&line).expect("No match found");
    let x: i64 = captures[1].parse().unwrap();
    let y: i64 = captures[2].parse().unwrap();
    return (x, y);
}


fn get_tokens(a_buttons: &Vec<(i64, i64)>, b_buttons: &Vec<(i64, i64)>, prizes: &Vec<(i64, i64)>) -> i64{
    let mut tokens = 0;
    for i in 0..prizes.len(){
        let (x1, y1) = a_buttons[i];
        let (x2, y2) = b_buttons[i];
        let (x, y) = prizes[i];

        if x1 * y2 == y1 * x2{continue;}
        let b = (y * x1 - x * y1) / (y2 * x1 - x2 * y1);
        let a = (x - b * x2) / x1;
        if a * x1 + b * x2 == x && a * y1 + b * y2 == y{
            tokens += 3 * a + b;
        }
    }
    return tokens;
}


pub fn day_13() -> io::Result<()> {
    let file = File::open("data/day13.aoc")?;
    let reader = io::BufReader::new(file);

    let mut a_buttons: Vec<(i64, i64)> = Vec::new();
    let mut b_buttons: Vec<(i64, i64)> = Vec::new();
    let mut prizes: Vec<(i64, i64)> = Vec::new();

    let button_regex = Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();
    let prize_regex = Regex::new(r"X=(\d+), Y=(\d+)").unwrap();

    let mut n_line = 0;
    for line in reader.lines(){
        let line = line?;
        if line.len() == 0{n_line = 0;continue;}
        if n_line == 0{
            a_buttons.push(regex_numbers(line, &button_regex));
            n_line += 1;
        }
        else if n_line == 1{
            b_buttons.push(regex_numbers(line, &button_regex));
            n_line += 1;
        }
        else if n_line == 2{
            prizes.push(regex_numbers(line, &prize_regex));
        }
    }
    let tokens = get_tokens(&a_buttons, &b_buttons, &prizes);
    println!("Part 1 answer: {}", tokens);
    let mut new_prizes: Vec<(i64, i64)> = Vec::new();

    for (x, y) in prizes{
        new_prizes.push((x + 10000000000000, y + 10000000000000));
    }

    let tokens = get_tokens(&a_buttons, &b_buttons, &new_prizes);
    println!("Part 2 answer: {}", tokens);

    Ok(())
}
