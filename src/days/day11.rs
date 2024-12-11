use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;


fn blink(stones: HashMap<u128, u128>) -> HashMap<u128, u128>{
    let mut new_stones: HashMap<u128, u128> = HashMap::new();
    for (stone, n_stones) in stones{
        if stone == 0{
            *new_stones.entry(1).or_insert(0) += n_stones;
            continue;
        }
        else{
            let pwr = stone.ilog10();
            if pwr % 2 == 1{
                let mid = (pwr + 1) / 2;
                let first: u128 = stone / 10_u128.pow(mid); 
                let second: u128 = stone % 10_u128.pow(mid);
                *new_stones.entry(first).or_insert(0) += n_stones;
                *new_stones.entry(second).or_insert(0) += n_stones;
                continue;
            }
        }
        *new_stones.entry(stone * 2024).or_insert(0) += n_stones;
    }
    return new_stones;
}


pub fn day_11() -> io::Result<()> {
    let file = File::open("data/day11.aoc")?;
    let mut reader = io::BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line)?;
    
    let stones: Vec<u128> = line.trim().split(' ').map(|x| x.parse().unwrap()).collect();
    let mut hash_stones: HashMap<u128, u128> = HashMap::new();

    for stone in stones{
        *hash_stones.entry(stone).or_insert(0) += 1;
    }
    for i in 0..75{
        hash_stones = blink(hash_stones);
        if vec![24, 74].contains(&i){
            let mut n: u128 = 0;
            for (_, val) in &hash_stones{
                n += val;
            }
            let level = if i == 24{1} else if i == 74{2} else {i + 1};
            println!("Part {} answer: {}", level, n);
        }
    }
    
    Ok(())
}
