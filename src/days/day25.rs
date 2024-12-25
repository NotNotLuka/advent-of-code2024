use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;



pub fn day_25() -> io::Result<()> {
    let file = File::open("data/day25.aoc")?;
    let reader = io::BufReader::new(file);


    let mut keys: Vec<Vec<Vec<char>>> = Vec::new();
    let mut locks: Vec<Vec<Vec<char>>> = Vec::new();

    let mut tmp: Vec<Vec<char>> = Vec::new();
    for line in reader.lines(){
        let line = line?;
        if line.is_empty(){
            if tmp[0].clone().into_iter().map(|x| x == '#').all(|x| x){
                locks.push(tmp);
            }else{
                keys.push(tmp);
            }

            tmp = Vec::new();
            continue;
        }
        tmp.push(line.chars().collect());
    }

    let mut unique_keys: HashSet<Vec<i64>> = HashSet::new();
    for key in &keys{
        let mut height: Vec<i64> = vec![0; key[0].len()];
        for j in (0..key.len()).rev(){
            for i in 0..key[j].len(){
                if key[j][i] == '#'{
                    height[i] += 1;
                }
            }
        }
        unique_keys.insert(height);
    }


    let mut unique_locks: HashSet<Vec<i64>> = HashSet::new();
    for lock in &locks{
        let mut height: Vec<i64> = vec![0; lock[0].len()];
        for j in 0..lock.len(){
            for i in 0..lock[j].len(){
                if lock[j][i] == '#'{
                    height[i] += 1;
                }
            }
        }
        unique_locks.insert(height);
    }


    let mut matches = 0;
    for lock in &unique_locks{
        for key in &unique_keys{
            let mut fits = true;
            for i in 0..lock.len(){
                if (key[i] + lock[i]) as usize > locks[0].len(){
                    fits = false;
                    break;
                }
            }
            if fits{
                matches += 1;
            }
        }
    }

    println!("Part 1 answer: {}", matches);
    println!("Part 2 answer: {}", "🎅");
    
    Ok(())
}
