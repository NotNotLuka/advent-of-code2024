use std::fs::File;
use std::io::{self, BufRead};


fn count(grid: &Vec<Vec<char>>, options: Vec<Vec<Vec<i32>>>, code_chars: Vec<char>) -> i32 {

    let mut n = 0;
    for y in 0..grid.len(){
        for x in 0..grid[0].len(){
            for opt in &options{
                for ind in 0..opt.len(){
                    let diff = &opt[ind];
                    let xn = x as i32 + diff[0];
                    let yn = y as i32 + diff[1];
                    if xn < 0 || yn < 0 || grid.len() <= (yn as usize) || grid[0].len() <= (xn as usize){break;}
                    if grid[yn as usize][xn as usize] != code_chars[ind]{break;}
                    if ind == opt.len() - 1{n += 1;}
                }
            }
        }
    }

    return n;
}


pub fn day_4() -> io::Result<()> {
    let file = File::open("data/day4.aoc")?;
    let reader = io::BufReader::new(file);

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in reader.lines(){
        grid.push(line?.chars().collect());
    }
    let code = "XMAS";
    let code_chars: Vec<char> = code.chars().collect();

    let mut options: Vec<Vec<Vec<i32>>> = Vec::new();

    options.push((0..code.len()).map(|i| vec![0 as i32, -(i as i32)]).collect());
    options.push((0..code.len()).map(|i| vec![0 as i32, i as i32]).collect());
    options.push((0..code.len()).map(|i| vec![i as i32, 0]).collect());
    options.push((0..code.len()).map(|i| vec![-(i as i32), 0]).collect());
    options.push((0..code.len()).map(|i| vec![i as i32, i as i32]).collect());
    options.push((0..code.len()).map(|i| vec![-(i as i32), -(i as i32)]).collect());
    options.push((0..code.len()).map(|i| vec![i as i32, -(i as i32)]).collect());
    options.push((0..code.len()).map(|i| vec![-(i as i32), i as i32]).collect());

    let n = count(&grid, options, code_chars);
    println!("Part 1 answer: {}", n);

    let mut options: Vec<Vec<Vec<i32>>> = Vec::new();
    let tmp_opt: Vec<Vec<i32>> = vec![
                                            vec![0, 0], // A
                                            vec![-1, -1], // M
                                            vec![1, 1], // S
                                            vec![-1, 1], // M
                                            vec![1, -1], // S
                                         ];
    options.push(tmp_opt);
    
    let tmp_opt: Vec<Vec<i32>> = vec![
                                            vec![0, 0], // A
                                            vec![1, 1], // M
                                            vec![-1, -1], // S
                                            vec![-1, 1], // M
                                            vec![1, -1], // S
                                         ];
    options.push(tmp_opt);

    let tmp_opt: Vec<Vec<i32>> = vec![
                                            vec![0, 0], // A
                                            vec![1, 1], // M
                                            vec![-1, -1], // S
                                            vec![1, -1], // M
                                            vec![-1, 1], // S
                                         ];
    options.push(tmp_opt);


    let tmp_opt: Vec<Vec<i32>> = vec![
                                            vec![0, 0], // A
                                            vec![-1, -1], // M
                                            vec![1, 1], // S
                                            vec![1, -1], // M
                                            vec![-1, 1], // S
                                         ];
    options.push(tmp_opt);

    let code = "AMSMS";
    let code_chars: Vec<char> = code.chars().collect();

    let n = count(&grid, options, code_chars);
    println!("Part 2 answer: {}", n);


    Ok(())
}
