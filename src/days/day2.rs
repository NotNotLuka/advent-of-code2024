use std::fs::File;
use std::io::{self, BufRead};


fn part_1(reader: io::BufReader<File>) -> io::Result<i32> {
    let mut n = 0;

    for line in reader.lines() {
        n += 1;
        let line = line?;
        let levels: Vec<&str> = line.split_whitespace().collect();

        let mut prev: i32 = levels[0].parse::<i32>().unwrap();
        let up_down: bool = 0 < (levels[1].parse::<i32>().unwrap() - levels[0].parse::<i32>().unwrap());

        for ind in 1..levels.len() {
            let cur: i32 = levels[ind].parse::<i32>().unwrap();
            if (0 < cur - prev) == up_down {
                if !(1 <= (cur - prev).abs() && (cur - prev).abs() <= 3) {
                    n -= 1;
                    break;
                }
            } else {
                n -= 1;
                break;
            }
            prev = cur;
        }
    }

    Ok(n)
}

fn check_levels(levels: Vec<&str>) -> bool {
    let mut prev: i32 = levels[0].parse::<i32>().unwrap();
    let up_down: bool = 0 < (levels[1].parse::<i32>().unwrap() - levels[0].parse::<i32>().unwrap());

    for ind in 1..levels.len() {
        let cur: i32 = levels[ind].parse::<i32>().unwrap();
        if (0 < cur - prev) == up_down {
            if !(1 <= (cur - prev).abs() && (cur - prev).abs() <= 3) {
                return false;
            }
        } else {
            return false;
        }
        prev = cur;
    }
    return true;
}

fn part_2(reader: io::BufReader<File>) -> io::Result<i32> {
    let mut n = 0;

    for line in reader.lines() {

        let line = line?;
        let levels: Vec<&str> = line.split_whitespace().collect();
        
        let mut good: bool = check_levels(levels.clone());
        if !good {
            for ind in 0..levels.len(){
                let mut new_levels = levels[0..ind].to_vec();
                new_levels.extend_from_slice(&levels[ind+1..]);
                let new_good: bool = check_levels(new_levels.clone());
                if new_good {
                    good = true;
                    break;
                }

            }
        }
        if good{n += 1;}
    }

    Ok(n)

}


pub fn day_2() -> io::Result<()> {
    let file = File::open("data/day2.aoc")?;
    let reader = io::BufReader::new(file);
    
    let n = part_1(reader)?;
    println!("Part 1 answer: {}", n);

    let file = File::open("data/day2.aoc")?;
    let reader = io::BufReader::new(file);
    let n = part_2(reader)?;
    println!("Part 2 answer: {}", n);

    Ok(())
}

