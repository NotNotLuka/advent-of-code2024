use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;



pub fn day_8() -> io::Result<()> {
    let file = File::open("data/day8.aoc")?;
    let reader = io::BufReader::new(file);


    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut mapped: Vec<Vec<char>> = Vec::new();

    for (y, line) in reader.lines().enumerate(){
        let line = line?;
        let line_vec: Vec<char> = line.chars().collect();
        for (x, ch) in line_vec.iter().enumerate(){
            if *ch != '.'{
                antennas.entry(*ch)
                        .or_insert_with(Vec::new)
                        .push((x as i32, y as i32));
            }
        }
        mapped.push(line_vec);
    }


    let mut antinodes: Vec<Vec<bool>> = mapped.iter()
        .map(|inner| inner.iter().map(|_| false).collect())  
        .collect();

    let mut antinodes2: Vec<Vec<bool>> = mapped.iter()
        .map(|inner| inner.iter().map(|_| false).collect())  
        .collect();
    for (_, value) in antennas {
        let mut distances: Vec<Vec<Vec<(i32, (i32, i32))>>> = Vec::new();
        for (x, y) in value{
            let mut distance: Vec<Vec<(i32, (i32, i32))>> = mapped.iter()
                .map(|inner| inner.iter().map(|_| (0, (0, 0))).collect())  
                .collect();
            for j in 0..distance.len(){
                for i in 0..distance[0].len(){
                    let dx = x - (i as i32);
                    let dy = y - (j as i32);
                    let dist = dx.pow(2) + dy.pow(2);
                    distance[j][i] = (dist, (dx, dy));
                }
            }
            distances.push(distance);
        }

        for n in 0..distances.len(){
            for n_2 in (n + 1)..distances.len(){
                for j in 0..distances[n].len(){
                    for i in 0..distances[n][0].len(){
                        let (dist_1, (dx1, dy1)) = distances[n][j][i];
                        let (dist_2, (dx2, dy2)) = distances[n_2][j][i];
                        if  (dy1 == 0 && dy2 == 0) || 
                            (dx1 == 0 && dy1 == 0) || (dx2 == 0 && dy2 == 00) ||
                            (dx1 as f64) / (dy1 as f64) == (dx2 as f64) / (dy2 as f64){
                            if dist_1 * 4 == dist_2 || dist_1 == dist_2 * 4{
                                antinodes[j][i] = true;
                            }
                            antinodes2[j][i] = true;
                        }
                    }
                }

            }
        }


    }

    let true_count = antinodes.iter()
        .flat_map(|row| row.iter()) 
        .filter(|&&b| b)
        .count();
    println!("Part 1 answer: {}", true_count);


    let true_count = antinodes2.iter()
        .flat_map(|row| row.iter()) 
        .filter(|&&b| b)
        .count();
    println!("Part 2 answer: {}", true_count);
    Ok(())
}
