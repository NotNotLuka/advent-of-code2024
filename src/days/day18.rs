use std::fs::File;
use std::io::{self, BufRead};


fn search_shortest(grid: &mut Vec<Vec<bool>>) -> i64{
    let neigh = vec![(-1, 0), (1, 0), (0, 1), (0, -1)];
    let mut go_to = vec![(0, 0)]; 
    let mut n = 0;
    while go_to.len() != 0{
        let mut new_go_to: Vec<(i64, i64)> = Vec::new();
        n += 1;
        for (x, y) in &go_to{
            for (i, j) in &neigh{
                if !(0 <= i + x && i + x < grid[0].len() as i64 
                    && 0 <= j + y && j + y < grid.len() as i64){continue;}
                if y + j == (grid.len() - 1) as i64 && x + i == (grid[0].len() - 1) as i64{
                    return n;
                }
                else if grid[(y + j) as usize][(x + i) as usize]{
                    new_go_to.push((x + i, y + j));
                    grid[(y + j) as usize][(x + i) as usize] = false;
                }
            }
        }
        go_to = new_go_to;
    }
    return -1;
}


pub fn day_18() -> io::Result<()> {
    let file = File::open("data/day18.aoc")?;
    let reader = io::BufReader::new(file);

    let mut grid = vec![vec![true; 71]; 71];

    let mut x = 0;
    for line in reader.lines(){
        let line = line?;
        let coords: Vec<i64> = line.split(',').map(|x| x.parse().unwrap()).collect();
        grid[coords[1] as usize][coords[0] as usize] = false;
        x += 1;
        if x == 1024{
            let n = search_shortest(&mut grid.clone());
            println!("Part 1 answer: {}", n);
        }
        if 1024 < x{
            let n = search_shortest(&mut grid.clone());
            if n == -1{
                println!("Part 2 answer: {},{}", coords[0], coords[1]);
                break;
            }
        }
    }
    Ok(())
}
