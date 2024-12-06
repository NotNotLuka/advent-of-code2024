use std::fs::File;
use std::io::{self, BufRead};


fn check_for_loop(start: (i32, i32), dir: (i32, i32), mapped: &Vec<Vec<char>>) -> bool {
    let mut dir = dir;
    let mut pos = start;
    let mut been_to: Vec<Vec<(bool, (i32, i32))>> = vec![vec![(false, (-1, -1)); mapped[0].len()]; mapped.len()];
    loop {
        let new_pos = ((pos.0 as i32) + dir.0, (pos.1 as i32) + dir.1);
        if !(0 <= new_pos.0 && (new_pos.0 as usize) < mapped[0].len() && 0 <= new_pos.1 && (new_pos.1 as usize) < mapped.len()){
            return false;
        }
        let (been, prev_dir) = been_to[new_pos.1 as usize][new_pos.0 as usize];
        if been && prev_dir == dir{return true;}
        if mapped[new_pos.1 as usize][new_pos.0 as usize] == '#'{
            match dir {
                (0, -1) => dir = (1, 0),
                (1, 0) => dir = (0, 1),
                (0, 1) => dir = (-1, 0),
                (-1, 0) => dir = (0, -1),
                _ => eprintln!("{}", "Unknown direction"),
            }
        }
        else{
            pos = new_pos;
            been_to[pos.1 as usize][pos.0 as usize] = (true, dir);
        }
    }
}


pub fn day_6() -> io::Result<()> {
    let file = File::open("data/day6.aoc")?;
    let reader = io::BufReader::new(file);

    let mut full_map: Vec<Vec<char>> = Vec::new();
    let mut start = (0, 0);
    for (ind, line) in reader.lines().enumerate(){
        let line = line.unwrap();
        full_map.push(line.chars().collect());
        match line.chars().position(|c| c == '^') {
            Some(index) => start = (index as i32, ind as i32),
            None => (),
        }
    }
    let mut pos = start;
    let mut been_to: Vec<Vec<bool>> = vec![vec![false; full_map[0].len()]; full_map.len()];
    been_to[pos.1 as usize][pos.0 as usize] = true;
    let mut dir: (i32, i32) = (0, -1);
    loop {
        let new_pos = ((pos.0 as i32) + dir.0, (pos.1 as i32) + dir.1);
        if !(0 <= new_pos.0 && (new_pos.0 as usize) < full_map[0].len() && 0 <= new_pos.1 && (new_pos.1 as usize) < full_map.len()){
            break;
        }
        if full_map[new_pos.1 as usize][new_pos.0 as usize] == '#'{
            match dir {
                (0, -1) => dir = (1, 0),
                (1, 0) => dir = (0, 1),
                (0, 1) => dir = (-1, 0),
                (-1, 0) => dir = (0, -1),
                _ => eprintln!("{}", "Unknown direction"),
            }
        }
        else{
            pos = new_pos;
            been_to[pos.1 as usize][pos.0 as usize] = true;
        }
    }

    let count = been_to 
    .iter()
    .flat_map(|row| row.iter())
    .filter(|&&val| val)
    .count();
    println!("Part 1 answer: {}", count);

    let mut count = 0;
    for j in 0..been_to.len(){
        for i in 0..been_to[0].len(){
            if !been_to[j][i]{continue;}
            let mut new_map = full_map.clone();
            new_map[j][i] = '#';
            if check_for_loop(start, (0, -1), &new_map) {
                count += 1;
            }
        }
    }
    println!("Part 2 answer: {}", count);

    Ok(())

}
