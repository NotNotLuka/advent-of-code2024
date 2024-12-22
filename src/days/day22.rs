use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;
use std::collections::HashMap;


fn get_next_number(cur: i64) -> i64{
    let a = cur ^ (cur * 64) % 16777216;
    let b = a ^ (a / 32) % 16777216;
    let c = b ^ (b * 2048) % 16777216;
    return c;
}

pub fn day_22() -> io::Result<()> {
    let file = File::open("data/day22.aoc")?;
    let reader = io::BufReader::new(file);

    let mut secrets: Vec<i64> = Vec::new();
    for line in reader.lines(){
        let line = line?;
        secrets.push(line.trim().parse().unwrap());

    }

    let mut sequences: Vec<Vec<i64>> = Vec::new(); 
    let mut result = 0;
    for i in 0..secrets.len(){
        let mut n = secrets[i];
        let mut seq = vec![n % 10];
        for _ in 0..2000{
            n = get_next_number(n);
            seq.push(n % 10);
        }
        sequences.push(seq);
        result += n;
    }
    println!("Part 1 answer: {}", result);

    let mut changes: Vec<Vec<i64>> = Vec::new();


    for seq in &sequences{
        let mut diff: Vec<i64> = Vec::new();
        let mut prev = seq[0];
        for x in seq[1..].to_vec(){
            diff.push(x - prev);
            prev = x;
        }
        changes.push(diff);
    }


    let mut banana: HashMap<Vec<i64>, i64> = HashMap::new();
    let mut all_options: Vec<HashSet<Vec<i64>>> = vec![HashSet::new(); sequences.len()];

    for i in 3..changes[0].len(){
        for j in 0..changes.len(){
            let seq = changes[j][(i-3)..(i+1)].to_vec();
            let bananas = sequences[j][i+1];
            if !all_options[j].contains(&seq){
                *banana.entry(seq.clone()).or_insert(0) += bananas;
                all_options[j].insert(seq);
            }
        }
    }

    let mut best = 0;
    for (_, banan) in &banana{
        if best < *banan{best = *banan;}
    }

    println!("Part 2 answer: {}", best);


    Ok(())
}
