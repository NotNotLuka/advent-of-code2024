use std::fs::File;
use std::io::{self, BufRead};


fn concat(a: i64, b: i64) -> i64 {
    a * 10_i64.pow(b.ilog10() + 1) + b
}

fn solve_eq(xs: &Vec<i64>, y: i64, first: Option<bool>) -> bool{
    let first = first.unwrap_or(true);
    let cur = xs[0];
    if y < cur{return false;}
    if xs.len() == 1{
        return cur == y;
    }

    let new = &xs[1..].to_vec();
    let mut mul = new.clone();
    mul[0] = cur * mul[0];
    let mut plus = new.clone();
    plus[0] = cur + plus[0];

    let part1 = solve_eq(&mul, y, Some(first)) || solve_eq(&plus, y, Some(first));
    if first{return part1;}

    let mut combine = new.clone();
    combine[0] = concat(cur, combine[0]);
    return part1 || solve_eq(&combine, y, Some(first));
}


pub fn day_7() -> io::Result<()> {
    let file = File::open("data/day7.aoc")?;
    let reader = io::BufReader::new(file);

    let mut equations: Vec<(i64, Vec<i64>)> = Vec::new();

    for line in reader.lines(){
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(':').collect();
        let (val, inputs) = (parts[0], parts[1]);
        let y: i64 = val.trim().parse().unwrap();
        let xs = inputs.trim().split(' ').map(|x| x.parse().unwrap()).collect();
        equations.push((y, xs));
    }

    let mut n = 0;
    for (y, xs) in equations.clone(){
        if solve_eq(&xs, y, None){n += y;}
    }
    println!("Part 1 answer: {}", n);

    let mut n = 0;
    for (y, xs) in equations.clone(){
        if solve_eq(&xs, y, Some(false)){n += y;}
    }
    println!("Part 2 answer: {}", n);
    Ok(())
}
