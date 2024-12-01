use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;


pub fn day_1() -> io::Result<()> {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();


    let file = File::open("data/day1.aoc")?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let words: Vec<&str> = line.split_whitespace().collect();

        left.push(words[0].parse().unwrap());
        right.push(words[1].parse().unwrap());
    }
    left.sort();
    right.sort();

    let mut answer = 0;

    for ind in 0..left.len() {
        let diff = left[ind] - right[ind];
        answer += diff.abs();
    }
    println!("Part 1: {}", answer);

    let mut n_appearences = HashMap::new();

    for &n in &right {
        match n_appearences.get(&n) {
            Some(&value) => {n_appearences.insert(n, value + 1);},
            None => {n_appearences.insert(n, 1);},
        }
    }

    let mut answer2 = 0;

    for &n in &left {
        match n_appearences.get(&n) {
            Some(&times) => {answer2 += n * times;},
            None => (),

        }
    }

    println!("Part 2: {}", answer2);

    Ok(())
}

