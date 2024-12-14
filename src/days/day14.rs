use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;


fn regex_line(line: String) -> ((i64, i64), (i64, i64)){
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    let captures = re.captures(&line).expect("No match found");
    let x: i64 = captures[1].parse().unwrap();
    let y: i64 = captures[2].parse().unwrap();
    let vx: i64 = captures[3].parse().unwrap();
    let vy: i64 = captures[4].parse().unwrap();
    return ((x, y), (vx, vy));
}

pub fn day_14() -> io::Result<()> {
    let file = File::open("data/day14.aoc")?;
    let reader = io::BufReader::new(file);

    let mut robots: Vec<((i64, i64), (i64, i64))> = Vec::new();

    for line in reader.lines(){
        let line = line?;
        robots.push(regex_line(line));
    }


    let mut quadrants = [[0, 0], [0 ,0]];
    let t: i64 = 486;
    let width: i64 = 101;
    let height: i64 = 103;
    let mut vec_2d = vec![vec![0; width as usize]; height as usize];

    for ((x, y), (vx, vy)) in &robots{
        let mut x_pos = (x + vx * t) % width;
        let mut y_pos = (y + vy * t) % height;
        if x_pos < 0{x_pos = width + x_pos;}
        if y_pos < 0{y_pos = height + y_pos;}
        vec_2d[y_pos as usize][x_pos as usize] += 1;
        let y_quadr: i64 = if y_pos < (height / 2) { 0 } else if height / 2 < y_pos{1} else{-1};
        let x_quadr: i64 = if x_pos < (width / 2) { 0 } else if width / 2 < x_pos{1} else{-1};
        if y_quadr == -1 || x_quadr == -1{continue;}
        quadrants[y_quadr as usize][x_quadr as usize] += 1;
    }
    let product = quadrants.iter()
                     .flat_map(|row| row.iter())
                     .product::<i64>();
    println!("Part 1 answer: {}", product);
    let mut n = 0;
    let mut cur_robots = robots.clone();
    loop{
        n += 1;
        let mut quadrants = [[0, 0], [0 ,0]];
        let mut new_robots = Vec::new();
        let mut vec_2d = vec![vec![0; width as usize]; height as usize];
        for ((x, y), (vx, vy)) in &cur_robots{
            let mut x_pos = (x + vx) % width;
            let mut y_pos = (y + vy) % height;
            if x_pos < 0{x_pos = width + x_pos;}
            if y_pos < 0{y_pos = height + y_pos;}
            vec_2d[y_pos as usize][x_pos as usize] += 1;
            new_robots.push(((x_pos, y_pos), (*vx, *vy)));
            let y_quadr: i64 = if y_pos < (height / 2) { 0 } else if height / 2 < y_pos{1} else{-1};
            let x_quadr: i64 = if x_pos < (width / 2) { 0 } else if width / 2 < x_pos{1} else{-1};
            if y_quadr == -1 || x_quadr == -1{continue;}

            quadrants[y_quadr as usize][x_quadr as usize] += 1;
        }
        cur_robots = new_robots.clone();
        if (quadrants[0][0] as f64) / (robots.len() as f64) > 0.4 ||
            (quadrants[0][1] as f64) / (robots.len() as f64) > 0.4 ||
            (quadrants[1][0] as f64) / (robots.len() as f64) > 0.4 ||
            (quadrants[0][0] as f64) / (robots.len() as f64) > 0.4 {
            println!("{}s", n);
            for vec in vec_2d{
                println!("{:?}", vec.into_iter()
                    .map(|num| if num == 0 { ' ' } else { '█' })
                    .collect::<Vec<char>>()
                    .iter()
                    .collect::<String>());
            }
        }
        if 10403 < n{break;}
    }
    Ok(())
}
