use std::fs::File;
use std::io::{self, BufRead};

fn empty_space(memory: &Vec<(i64, i64)>) -> bool{
    for i in 0..(memory.len() - 1){
        let (_, mem) = memory[i];
        if mem == -1{return true;}
    }
    return false;
}


fn check_sum(memory: &Vec<(i64, i64)>) -> i64{
    let mut checksum = 0;
    let mut ind = 0;
    for (x, mem) in memory{
        if *mem == -1{ind += x; continue;}
        for i in ind..(ind + x){
            checksum += i * mem;
        }
        ind += x;
    }
    return checksum;
}

fn part_1(mut memory: Vec<(i64, i64)>) -> i64{
    while empty_space(&memory){
        for (ind, (x, mem)) in memory.iter().enumerate(){
            let mem = *mem;
            let x = *x;
            if mem != -1{continue;}
            let (last_x, last_mem) = memory[memory.len() - 1];
            if last_x < x{
                memory[ind as usize] = (x - last_x, -1);
                memory.pop();
                memory.insert(ind as usize, (last_x, last_mem));
            }
            else{
                memory.remove(ind as usize);
                memory.insert(ind as usize, (x, last_mem));
                let len = memory.len();
                if last_x != x{
                    memory[len - 1] = (last_x - x, last_mem);
                }
                else{
                    memory.pop();
                }
            }
            break;
        }

    }
    return check_sum(&memory);
}


fn part_2(mut memory: Vec<(i64, i64)>) -> i64{
    let iterator = memory.clone().into_iter().rev();
    for (last_x, last_mem) in iterator {
        if last_mem == -1{continue;}
        for (ind, (x, mem)) in memory.iter_mut().enumerate(){
            let x = *x;
            let mem = *mem;
            if mem != -1{continue;}
            if last_x <= x{
                let remove_ind = memory.iter().position(|&(_, snd)| snd==last_mem).unwrap();
                if remove_ind < ind{break;}
                memory[remove_ind] = (last_x, -1);
                if x != last_x{
                    memory[ind as usize] = (x - last_x, -1);
                }
                else{memory.remove(ind);}
                
                memory.insert(ind as usize, (last_x, last_mem));
                break;
            }
        
        }


    }
    return check_sum(&memory);
}

pub fn day_9() -> io::Result<()> {
    let file = File::open("data/day9.aoc")?;
    let mut reader = io::BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line)?;
    let block: Vec<i64> = line
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect();

    let mut memory: Vec<(i64, i64)> = Vec::new();
    let mut n = 0;
    for (ind, x) in block.iter().enumerate(){
        if *x == 0{continue;}
        if ind % 2 == 0{
            memory.push((*x, n));
            n += 1;
        }
        else{
            memory.push((*x, -1));
        }
    }

    let checksum = part_1(memory.clone());
    println!("Part 1 answer: {}", checksum);

    let checksum = part_2(memory.clone());
    println!("Part 2 answer: {}", checksum);

    Ok(())
}
