use std::fs::File;
use std::io::{self, BufRead};

fn check_ahead(positions: &Vec<(i64, i64)>, mapped: &Vec<Vec<char>>) -> char{
    let mut all = true;
    for (x, y) in positions{
        if mapped[*y as usize][*x as usize] != '.'{
            all = false;
        }
        if mapped[*y as usize][*x as usize] == '#'{
            return '#';
        }
    }
    if all{
        return '.'
    }
    return 'O'; // irrelevant

}


fn make_fat_move(start_pos: (i64, i64), direction: char, mapped: &mut Vec<Vec<char>>) -> (i64, i64){
    let dir: (i64, i64);
    match direction {
        '^' => {dir = (0, -1);}
        'v' => {dir = (0, 1);}
        '>' => {dir = (1, 0);}
        '<' => {dir = (-1, 0);}
        _ => {panic!("Unknown value");}
    }

    let mut positions = vec![start_pos];
    let mut to_move = vec![start_pos];
    let mut move_ahead: char = check_ahead(&positions, mapped);

    while move_ahead != '.'{
        if move_ahead == '#'{
            return start_pos;
        }

        for (x, y) in positions.clone(){
            if mapped[y as usize][x as usize] == '['{
                positions.push((x+1, y));
                if dir == (1, 0){
                    positions.retain(|&z| z != (x, y));
                    to_move.push((x, y));
                }
            }
            else if mapped[y as usize][x as usize] == ']'{
                positions.push((x-1, y));
                if dir == (-1, 0){
                    positions.retain(|&z| z != (x, y));
                    to_move.push((x, y));
                }
            }
        }
        let mut new_positions: Vec<(i64, i64)> = Vec::new();
        for (x, y) in &positions{
            if mapped[*y as usize][*x as usize] != '.'{
                let new_pos = (*x + dir.0, *y + dir.1);
                to_move.push((*x, *y));
                new_positions.push(new_pos);
            }
            else{
                new_positions.push((*x, *y));
            }
        }
        positions = new_positions;
        move_ahead = check_ahead(&positions, mapped);
    }

    let mut new_map = mapped.clone();
    let mut reset: Vec<(i64, i64)> = Vec::new();
    let mut moved_to: Vec<(i64, i64)> = Vec::new();
    for (x, y) in &to_move{
        let new_pos = (*x + dir.0, *y + dir.1);
        new_map[new_pos.1 as usize][new_pos.0 as usize] = mapped[*y as usize][*x as usize];
        reset.push((*x, *y));
        moved_to.push(new_pos);
    }

    for coord in &reset{
        if moved_to.contains(&coord){continue;}
        new_map[coord.1 as usize][coord.0 as usize] = '.';
    }
    for y in 0..new_map.len(){
        for x in 0..new_map[0].len(){
            mapped[y][x] = new_map[y][x];
        }
    }
    let new_pos = (start_pos.0 + dir.0, start_pos.1 + dir.1);
    return new_pos;
}

fn make_move(start_pos: (i64, i64), direction: char, mapped: &mut Vec<Vec<char>>) -> (i64, i64){
    let dir: (i64, i64);
    match direction {
        '^' => {dir = (0, -1);}
        'v' => {dir = (0, 1);}
        '>' => {dir = (1, 0);}
        '<' => {dir = (-1, 0);}
        _ => {panic!("Unknown value");}
    }

    let mut pos = start_pos;
    while mapped[pos.1 as usize][pos.0 as usize] != '.'{
        if mapped[pos.1 as usize][pos.0 as usize] == '#'{
            return start_pos;
        }
        pos = (pos.0 + dir.0, pos.1 + dir.1);
    }

    mapped[pos.1 as usize][pos.0 as usize] = 'O';
    mapped[start_pos.1 as usize][start_pos.0 as usize] = '.';
    let new_pos = (start_pos.0 + dir.0, start_pos.1 + dir.1);
    mapped[new_pos.1 as usize][new_pos.0 as usize] = '@';
    return new_pos;
}


pub fn day_15() -> io::Result<()> {
    let file = File::open("data/day15.aoc")?;
    let reader = io::BufReader::new(file);

    let mut mapped: Vec<Vec<char>> = Vec::new();
    let mut instructions: Vec<char> = Vec::new();
    let mut empty = false;
    for line in reader.lines(){
        let line = line?;
        if line.trim().len() == 0{empty = true; continue;}
        if !empty{
            mapped.push(line.trim().chars().collect());
        }
        else{
            let chars: Vec<char> = line.trim().chars().collect();
            instructions.extend(chars);
        }
    }

    let mut start_pos: (i64, i64) = (-1, -1);
    for y in 0..mapped.len(){
        for x in 0..mapped[0].len(){
            if mapped[y][x] == '@'{
                start_pos = (x as i64, y as i64);
                break;
            }
        }
    }
    let mut pos = start_pos;
    let mut part_1_map = mapped.clone();
    for instruct in &instructions{
        pos = make_move(pos, *instruct, &mut part_1_map);
    }
    let mut n = 0;

    for y in 0..part_1_map.len(){
        for x in 0..part_1_map[0].len(){
            if part_1_map[y][x] == 'O'{
                n += x + 100 * y;
            }
        }
    }

    println!("Part 1 answer: {}", n);

    let mut part_2_map: Vec<Vec<char>>= Vec::new();
    for y in 0..mapped.len(){
        let mut tmp: Vec<char> = Vec::new();
        for x in 0..mapped[0].len(){
            if mapped[y][x] == '#'{
                tmp.push('#');
                tmp.push('#');
            } 
            else if mapped[y][x] == '.'{
               tmp.push('.'); 
               tmp.push('.'); 
            }else if mapped[y][x] == 'O'{
                tmp.push('[');
                tmp.push(']');
            } else if mapped[y][x] == '@'{
                tmp.push('@');
                tmp.push('.');
            }

        }
        part_2_map.push(tmp);
    }
    for y in 0..part_2_map.len(){
        for x in 0..part_2_map[0].len(){
            if part_2_map[y][x] == '@'{
                start_pos = (x as i64, y as i64);
                break;
            }
        }
    }

    let mut pos = start_pos;
    for instruct in &instructions{
        pos = make_fat_move(pos, *instruct, &mut part_2_map);
    }

    let mut n = 0;

    for y in 0..part_2_map.len(){
        for x in 0..part_2_map[0].len(){
            if part_2_map[y][x] == '['{
                n += x + 100 * y;
            }
        }
    }
    println!("Part 2 answer: {}", n);
    Ok(())
}
