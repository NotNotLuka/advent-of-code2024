use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use itertools::Itertools;




fn dir_to_char(directions: &Vec<(i64, i64)>) -> Vec<char>{
    let mut out: Vec<char> = Vec::new();
    for dir in directions{
        match dir {
            (-1, 0) => {out.push('<')},
            (1, 0) => {out.push('>')},
            (0, 1) => {out.push('v')},
            (0, -1) => {out.push('^')},
            _ => panic!("Unknown direction"),
        }
    }
    out.push('A');
    return out;
}


fn find_paths(pos: (i64, i64), names: &HashMap<(i64, i64), char>, grid: &mut Vec<Vec<bool>>) -> HashMap<char, Vec<char>>{
    let mut connections: HashMap<char, Vec<char>> = HashMap::new();
    let neigh = vec![(-1, 0), (1, 0), (0, 1), (0, -1)];
    connections.insert(*names.get(&pos).unwrap(), dir_to_char(&Vec::new()));
    grid[pos.1 as usize][pos.0 as usize] = false;

    let mut go_to: Vec<((i64, i64), Vec<(i64, i64)>)> = vec![(pos, Vec::new())]; 
    while go_to.len() != 0{
        let mut new_go_to: Vec<((i64, i64), Vec<(i64, i64)>)> = Vec::new();
        for ((x, y), cur_dir) in &go_to{
            for (i, j) in &neigh{
                let (i, j) = (*i, *j);
                if !(0 <= i + x && i + x < grid[0].len() as i64 
                    && 0 <= j + y && j + y < grid.len() as i64){continue;}
                if grid[(y + j) as usize][(x + i) as usize]{
                    let mut tmp_dir = cur_dir.clone();
                    tmp_dir.push((i, j));
                    new_go_to.push(((x + i, y + j), tmp_dir.clone()));
                    grid[(y + j) as usize][(x + i) as usize] = false;

                    connections.insert(*names.get(&(x+i, y+j)).unwrap(), dir_to_char(&tmp_dir));
                }
            }
        }
        go_to = new_go_to;
    }
    return connections;
}

fn define_digits(digits: &HashMap<(i64, i64), char>, grid: &Vec<Vec<bool>>) -> HashMap<char, HashMap<char, Vec<char>>>{


    let mut all_paths: HashMap<char, HashMap<char, Vec<char>>> = HashMap::new();
    for (pos, digit) in digits{
        let paths: HashMap<char, Vec<char>> = find_paths(*pos, &digits, &mut grid.clone());
        all_paths.insert(*digit, paths);
    }

    return all_paths;
}


fn define_pad(pads: &HashMap<(i64, i64), char>, grid: &Vec<Vec<bool>>) -> HashMap<char, HashMap<char, Vec<char>>>{

    let mut all_paths: HashMap<char, HashMap<char, Vec<char>>> = HashMap::new();
    for (pos, pad) in pads{
        let paths: HashMap<char, Vec<char>> = find_paths(*pos, &pads, &mut grid.clone());
        all_paths.insert(*pad, paths);
    }
        
    return all_paths;
}


fn verify_perm(start: (i64, i64), perm: Vec<char>, grid: &Vec<Vec<bool>>) -> bool {
    let (mut x, mut y) = start;
    for dir in perm{
        let (i, j);
        match dir{
            '^' => {i=0;j=-1;},
            'v' => {i=0;j=1;},
            '>' => {i=1;j=0;},
            '<' => {i=-1;j=0},
            _ => panic!("Unknown direction"),
        }
        if !(0 <= x + i && 0 <= y + j && x + i < (grid[0].len() as i64) && (y + j) < (grid.len() as i64)){
            panic!("Shouldn't happen");
        }
        if !grid[(y + j) as usize][(x + i) as usize]{
            return false;
        }
        x += i;
        y += j;
    }
    return true;
}


fn normal_decode(code: Vec<char>, lookup: &HashMap<char, HashMap<char, Vec<char>>>) -> Vec<char>{
    let mut new_code: Vec<char> = Vec::new();
    let mut prev_pos = 'A';
    for pos in code{
        let moves = lookup.get(&prev_pos).unwrap().get(&pos).unwrap();
        new_code.extend(moves);
        prev_pos = pos;
    }

    return new_code;
}
fn sort_optimally(to_sort: &HashMap<char, HashMap<char, Vec<char>>>, remote: &HashMap<char, HashMap<char, Vec<char>>>, grid: &Vec<Vec<bool>>, names: &HashMap<(i64, i64), char>) -> HashMap<char, HashMap<char, Vec<Vec<char>>>>{
    let mut inverted: HashMap<char, (i64, i64)> = HashMap::new();
    for (key, value) in names {
        inverted.insert(*value, *key);
    }

    let mut sorted: HashMap<char, HashMap<char, Vec<Vec<char>>>> = HashMap::new();
    for (key1, amb) in to_sort{
        let mut sub_sorted: HashMap<char, Vec<Vec<char>>> = HashMap::new();
        for (key2, seq) in amb{
            let for_perms = seq[..(seq.len() - 1)].to_vec();
            let mut same_size: Vec<Vec<char>> = Vec::new(); 
            let mut smallest_size: i64 = i64::MAX;
            for perm in for_perms.iter().permutations(for_perms.len()).unique(){
                let mut perm: Vec<char> = perm.into_iter().cloned().collect();
                let valid = verify_perm(*inverted.get(&key1).unwrap(), perm.clone(), &grid);
                if !valid{continue;}
                perm.push('A');
                let decoded = normal_decode(perm.clone(), remote);
                let size = decoded.len() as i64;
                if size < smallest_size{
                    same_size = vec![perm];
                    smallest_size = size;
                }else if size == smallest_size && !same_size.contains(&perm){
                    same_size.push(perm);
                }

            }
            sub_sorted.insert(*key2, same_size);
        }
        sorted.insert(*key1, sub_sorted);
    }
    return sorted;
}


fn decode(prev: &char, cur: &char, depth: i64, 
    keypad: &HashMap<char, HashMap<char, Vec<Vec<char>>>>, 
    remote: &HashMap<char, HashMap<char, Vec<Vec<char>>>>,
    mem: &mut HashMap<(char, char, i64), i64>, 
    start_depth: i64) -> i64{
    if let Some(value) = mem.get(&(*prev, *cur, depth)) {
        return value.clone();
    }
    let lookup: &HashMap<char, HashMap<char, Vec<Vec<char>>>>;
    if depth == start_depth{
        lookup = keypad;
    }
    else{lookup = remote;}
    let moves = lookup.get(prev).unwrap().get(cur).unwrap();
    if depth == 0{
        let minimal = moves.into_iter().min_by_key(|v| v.len()).unwrap();
        return minimal.len() as i64;
    }

    let mut shortest = i64::MAX;
    for moved in moves{
        let mut prev_pos = &'A';
        let mut length = 0;
        for mv in moved{
            length += decode(prev_pos, mv, depth - 1, keypad, remote, mem, start_depth);
            prev_pos = mv;
        }
        if length < shortest{shortest = length;}
    }


    mem.insert((*prev, *cur, depth), shortest);
    return shortest;
}


pub fn day_21() -> io::Result<()> {
    let file = File::open("data/day21.aoc")?;
    let reader = io::BufReader::new(file);

    let mut codes: Vec<Vec<char>> = Vec::new();

    for line in reader.lines(){
        let line = line?;
        codes.push(line.trim().chars().collect());
    }

    let mut digits: HashMap<(i64, i64), char> = HashMap::new();
    digits.insert((2, 3), 'A');
    digits.insert((1, 3), '0');
    digits.insert((0, 2), '1');
    digits.insert((1, 2), '2');
    digits.insert((2, 2), '3');
    digits.insert((0, 1), '4');
    digits.insert((1, 1), '5');
    digits.insert((2, 1), '6');
    digits.insert((0, 0), '7');
    digits.insert((1, 0), '8');
    digits.insert((2, 0), '9');

    let mut digit_grid: Vec<Vec<bool>> = vec![vec![true; 3]; 4];
    digit_grid[3][0] = false;
    let keypad = define_digits(&digits, &digit_grid.clone());

    let mut pads: HashMap<(i64, i64), char> = HashMap::new();
    pads.insert((1, 0), '^');
    pads.insert((2, 0), 'A');
    pads.insert((0, 1), '<');
    pads.insert((1, 1), 'v');
    pads.insert((2, 1), '>');

    let mut pad_grid: Vec<Vec<bool>> = vec![vec![true; 3]; 2];
    pad_grid[0][0] = false;

    let remote = define_pad(&pads, &pad_grid.clone());
    let all_remote = sort_optimally(&remote, &remote, &pad_grid, &pads);
    let all_keypad = sort_optimally(&keypad, &remote, &digit_grid, &digits);
    let mut mem: HashMap<(char, char, i64), i64> = HashMap::new();
    let mut complexity2: i64 = 0;
    let mut complexity25: i64 = 0;
    for code in codes{
        let mut length2 = 0;
        let mut length25 = 0;
        let mut prev = &'A';
        for chr in &code{
            let dist2 = decode(prev, chr, 2, &all_keypad, &all_remote, &mut mem, 2); 
            let dist25 = decode(prev, chr, 25, &all_keypad, &all_remote, &mut mem, 25); 
            length2 += dist2;
            length25 += dist25;
            prev = chr;
        }


        let numerical: i64 = code.iter()
                                .take(3).collect::<String>()
                                .parse().unwrap();
        complexity2 += numerical * length2;
        complexity25 += numerical * length25;
    }


    println!("Part 1 answer: {}", complexity2);
    println!("Part 2 answer: {}", complexity25);

    Ok(())
}
