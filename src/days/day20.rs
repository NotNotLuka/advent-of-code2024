use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Reverse;


fn search_shortest(start: (i64, i64), grid: &mut Vec<Vec<bool>>) -> Vec<(i64, i64)>{
    let neigh = vec![(-1, 0), (1, 0), (0, 1), (0, -1)];
    let mut go_to = vec![start]; 
    let mut visited: Vec<(i64, i64)> = Vec::new();
    for _ in 0..20{
        let mut new_go_to: Vec<(i64, i64)> = Vec::new();
        for (x, y) in &go_to{
            for (i, j) in &neigh{
                if !(0 <= i + x && i + x < grid[0].len() as i64 
                    && 0 <= j + y && j + y < grid.len() as i64){continue;}
                if grid[(y + j) as usize][(x + i) as usize]{
                    new_go_to.push((x + i, y + j));
                    visited.push((x + i, y + j));
                    grid[(y + j) as usize][(x + i) as usize] = false;
                }
            }
        }
        go_to = new_go_to;
    }
    return visited;
}


fn part_2(shortest: i64, end: (usize, usize), grid: Vec<Vec<bool>>, from_start: HashMap<(usize, usize), i64>, from_end: HashMap<(usize, usize), i64>) -> HashMap<i64, i64>{
    let mut how_many: HashMap<i64, i64> = HashMap::new();
    let mut shortcuts: HashMap<((i64, i64), (i64, i64)), i64> = HashMap::new();
    for y in 0..grid.len(){
        for x in 0..grid[0].len(){
            if !grid[y][x]{continue;}
            let x = x as usize; let y = y as usize;

            let to_start = from_start.get(&(x as usize, y as usize)).unwrap();

            if (x as i64 - end.0 as i64).abs() + (y as i64 - end.1 as i64).abs() <= 20{
                let dist = to_start + (x as i64 - end.0 as i64).abs() + (y as i64 - end.1 as i64).abs();
                if dist < shortest{shortcuts.insert(((x as i64, y as i64), (end.0 as i64, end.1 as i64)), shortest - dist);}
            }
            let mut tmp_grid = vec![vec![true; grid[0].len()]; grid.len()];
            tmp_grid[y][x] = false;
            let visited = search_shortest((x as i64, y as i64), &mut tmp_grid);
            for (x_m, y_m) in visited{
                if !grid[y_m as usize][x_m as usize]{continue;}
                let to_end = from_end.get(&(x_m as usize, y_m as usize)).unwrap();
                if *to_start == i64::MAX || *to_end == i64::MAX{continue;}
                let dist = to_start + to_end + (x_m - x as i64).abs() + (y_m - y as i64).abs();
                if dist < shortest{shortcuts.insert(((x as i64, y as i64), (x_m, y_m)), shortest - dist);}
            }
        }
    }

    for (_, val) in shortcuts{
        *how_many.entry(val).or_insert(0) += 1;
    }
    return how_many;
}


fn djikstra(start_node: (usize, usize), mut scores: HashMap<(usize, usize), i64>, all_moves: &HashMap<(usize, usize), Vec<((usize, usize), i64)>>) -> HashMap<(usize, usize), i64>{
    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(Reverse((0, start_node.clone())));
    scores.insert(start_node, 0);

    while let Some(Reverse((score, node))) = priority_queue.pop(){
        if let Some(move_to_nodes) = all_moves.get(&node){
            for (move_to, add_score) in move_to_nodes{
                let cur_score = scores.get(move_to).unwrap();
                if score + add_score < *cur_score{
                    scores.insert(move_to.clone(), score + add_score);
                    priority_queue.push(Reverse((score + add_score, move_to.clone())));
                }
            }
        }
    }
    return scores;
}

pub fn day_20() -> io::Result<()> {
    let file = File::open("data/day20.aoc")?;
    let reader = io::BufReader::new(file);

    let mut mapped: Vec<Vec<char>> = Vec::new();

    for line in reader.lines(){
        let line = line?;
        mapped.push(line.chars().collect());
    }

    let mut grid: Vec<Vec<bool>> = vec![vec![true; mapped[0].len()]; mapped.len()];

    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    let mut cheats: Vec<(i64, i64)> = Vec::new();
    for y in 0..mapped.len(){
        for x in 0..mapped[0].len(){
            match mapped[y][x]{
                'S' => {start = (x as usize, y as usize);},
                'E' => {end = (x as usize, y as usize);}
                '.' => {grid[y][x] = true;},
                '#' => {grid[y][x] = false; cheats.push((x as i64, y as i64));}
                _ => panic!("Unknown character"),
            }
        }
    }
    let mut all_moves: HashMap<(usize, usize), Vec<((usize, usize), i64)>> = HashMap::new();
    let mut scores: HashMap<(usize, usize), i64> = HashMap::new();
    let neigh: Vec<(i64, i64)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    for y in 0..mapped.len(){
        for x in 0..mapped[0].len(){
            if grid[y][x]{
                scores.insert((x, y), i64::MAX);
            }
            for (i, j) in &neigh{
                let tmp_x = x as i64 + i;
                let tmp_y = y as i64 + j;
                if !(0 <= tmp_x && 0 <= tmp_y && (tmp_y as usize) < mapped.len() && (tmp_x as usize) < mapped[0].len()){continue;}
                if grid[tmp_y as usize][tmp_x as usize]{
                    all_moves
                    .entry((x, y))
                    .or_insert_with(Vec::new)
                    .push(((tmp_x as usize, tmp_y as usize), 1));
                }
            }
            
        }
    }

    let from_start: HashMap<(usize, usize), i64> = djikstra(start, scores.clone(), &all_moves);
    let shortest = from_start.get(&end).unwrap();
    let from_end: HashMap<(usize, usize), i64> = djikstra(end, scores.clone(), &all_moves);
    let mut how_many: HashMap<i64, i64> = HashMap::new();
    for cheat in &cheats{
        let mut minimal = i64::MAX;
        for k_neigh in 0..neigh.len(){
            let x = cheat.0 + neigh[k_neigh].0;
            let y = cheat.1 + neigh[k_neigh].1;
            if !(0 < x && 0 < y && (x as usize) < grid[0].len() && (y as usize) < grid.len()){continue;}
            let x = x as usize; let y = y as usize;
            if !grid[y][x]{continue;}
            for m_neigh in 0..neigh.len(){
                let x_m = cheat.0 + neigh[m_neigh].0;
                let y_m = cheat.1 + neigh[m_neigh].1;
                if !(0 < x_m && 0 < y_m && (x_m as usize) < grid[0].len() && (y_m as usize) < grid.len()){continue;}
                let x_m = x_m as usize; let y_m = y_m as usize;
                if !grid[y_m][x_m] || (x_m == x && y_m == y){continue;}

                let to_start = from_start.get(&(x, y)).unwrap();
                let to_end = from_end.get(&(x_m, y_m)).unwrap();
                if *to_start == i64::MAX || *to_end == i64::MAX{continue;}
                let dist = to_start + to_end + 2; 
                if dist < minimal{minimal = dist;}
            }
        }
        if *shortest < minimal{continue;}
        *how_many.entry(shortest - minimal).or_insert(0) += 1;
    }
    

    let mut sorted_keys: Vec<_> = how_many.keys().collect();
    sorted_keys.sort();
    let mut n = 0;
    for key in sorted_keys{
        if *key < 100{continue;}
        let value = how_many.get(&key).unwrap();
        n += value;
    }

    println!("Part 1 answer: {}", n);

    let answer_2 = part_2(*shortest, end, grid, from_start, from_end);

    let mut sorted_keys: Vec<_> = answer_2.keys().collect();
    sorted_keys.sort();
    let mut m = 0;
    for key in sorted_keys{
        let value = answer_2.get(&key).unwrap();
        if *key < 100{continue;}
        m += value;
    }
    println!("Part 2 answer: {}", m);

    Ok(())
}
