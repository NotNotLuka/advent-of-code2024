use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;


fn get_perimeter(group: &Vec<(i32, i32)>) -> i32{
    let neighs: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut perimeter = 0;
    for (x, y) in group{
        for (i, j) in &neighs{
            let new_x = x + i;
            let new_y = y + j;
            if !group.contains(&(new_x, new_y)){
                perimeter += 1;
            }
        }
    }

    return perimeter;
}

fn get_sides(group: &Vec<(i32, i32)>) -> i32{
    let neighs: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut sides = 0;
    let mut x_sides_left: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    let mut x_sides_right: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    let mut y_sides_up: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    let mut y_sides_down: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();

    for (x, y) in group{
        for (i, j) in &neighs{
            let (x, y) = (*x, *y);
            let new_x = x + i;
            let new_y = y + j;
            if !group.contains(&(new_x, new_y)){
                match (i, j){
                    (-1, 0) => {x_sides_left.entry(x).or_insert(Vec::new()).push((x, y))},
                    (1, 0) => {x_sides_right.entry(x).or_insert(Vec::new()).push((x, y))},
                    (0, 1) => {y_sides_down.entry(y).or_insert(Vec::new()).push((x, y))},
                    (0, -1) => {y_sides_up.entry(y).or_insert(Vec::new()).push((x, y))}
                    _ => unreachable!("Unexpected value: {}, {}", i, j)
                }
            }
        }
    }
    let y_sides = vec![y_sides_up, y_sides_down];
    let x_sides = vec![x_sides_left, x_sides_right];

    for map in y_sides{
        for (_, mut val) in map{
            val.sort_by_key(|&(x, _)| x);
            let mut last_x = -1;
            for (x, _) in val{
                if last_x == -1 || 1 < x - last_x{
                    sides += 1;
                } 
                last_x = x;
            }
        }

    }

    for map in x_sides{
        for (_, mut val) in map{
            val.sort_by_key(|&(_, y)| y);
            let mut last_y = -1;
            for (_, y) in val{
                if last_y == -1 || 1 < y - last_y{
                    sides += 1;
                } 
                last_y = y;
            }
        }

    }

    return sides;
}




fn part_1(mapped: &Vec<Vec<char>>) -> (i32, i32){

    let mut groups: Vec<Vec<(i32, i32)>> = Vec::new();
    let mut been_to: Vec<Vec<bool>> = mapped.into_iter().map(|x| x.into_iter().map(|_| false).collect()).collect();
    let mut go_to: Vec<(i32, i32)> = Vec::new();
    for i in 0..mapped[0].len(){for j in 0..mapped.len(){go_to.push((i as i32, j as i32));}}
    let neighs: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    for (x, y) in go_to{
        if been_to[y as usize][x as usize]{continue;}
        let mut group: Vec<(i32, i32)> = vec![(x, y)];
        let mut next_move: Vec<(i32, i32)> = vec![(x, y)];
        been_to[y as usize][x as usize] = true;
        while next_move.len() != 0{
            let (x, y) = next_move.pop().unwrap();
            for (i, j) in &neighs{
                let x_new = x + i;
                let y_new = y + j;
                if x_new < 0 || y_new < 0 || mapped.len() <= (y_new as usize) || mapped[0].len() <= (x_new as usize){continue;} 
                if !been_to[y_new as usize][x_new as usize] &&
                    mapped[y as usize][x as usize] == mapped[y_new as usize][x_new as usize]{
                        been_to[y_new as usize][x_new as usize] = true;
                        next_move.push((x_new, y_new));
                        group.push((x_new, y_new));
                }
            }
        }
        groups.push(group);
    }

    let mut n1 = 0;
    let mut n2 = 0;
    for group in groups{
        let area = group.len() as i32;
        let perimeter = get_perimeter(&group);
        let sides = get_sides(&group);
        n1 += perimeter * area;
        n2 += sides * area;
    }
    return (n1, n2);
}


pub fn day_12() -> io::Result<()> {
    let file = File::open("data/day12.aoc")?;
    let reader = io::BufReader::new(file);

    let mut mapped: Vec<Vec<char>> = Vec::new();

    for line in reader.lines(){
        let line = line?;
        mapped.push(line.chars().collect());
    }

    let (n1, n2) = part_1(&mapped);
    println!("Part 1 answer: {}", n1);
    println!("Part 2 answer: {}", n2);


    Ok(())
}
