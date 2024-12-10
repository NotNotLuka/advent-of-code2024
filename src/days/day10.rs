use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::collections::HashSet;

fn coords_to_string(nxt: (i32, i32)) -> String{
    return format!("{}-{}", nxt.0, nxt.1);
}

fn part_1(mapped: &Vec<Vec<i32>>) -> (i32, i32){
    let mut start: Vec<Vec<(i32, i32)>> = Vec::new();
    for (i, x) in mapped.iter().enumerate(){
        for (j, y) in x.iter().enumerate(){
            if *y == 0{start.push(vec![(j as i32, i as i32)]);}
        }
    }

    let mut to_do = start.clone();

    let neigh: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, 1), (0, -1)];
    let mut all_nines1: HashMap<String, HashSet<String>> = HashMap::new();
    let mut all_nines2: HashMap<String, i32> = HashMap::new();
    while to_do.len() != 0{
        let element = to_do.pop().unwrap();
        let cur = element[element.len() - 1];

        for mv in &neigh{
            let nxt = (mv.0 + cur.0, mv.1 + cur.1);
            if !(0 <= nxt.1 && (nxt.1 as usize) < mapped.len()
                && 0 <= nxt.0 && (nxt.0 as usize) < mapped[0].len()){continue;}

            if mapped[nxt.1 as usize][nxt.0 as usize] - mapped[cur.1 as usize][cur.0 as usize] == 1{
                let mut new_element = element.clone();  
                new_element.push(nxt);
                if mapped[nxt.1 as usize][nxt.0 as usize] == 9{
                    let begin = coords_to_string(new_element[0]);
                    let end = coords_to_string(new_element[new_element.len() - 1]);
                    all_nines1.entry(begin.clone()).or_insert_with(HashSet::new).insert(end);
                    *all_nines2.entry(begin.clone()).or_insert(0) += 1;
                }
                else{to_do.push(new_element);}
            }
        }
    }


    let mut n = 0;
    for (_, val) in all_nines1{
        n += val.len() as i32;
    }

    let mut m = 0;
    for (_, val) in all_nines2{
        m += val;
    }

    return (n, m);


}


pub fn day_10() -> io::Result<()> {
    let file = File::open("data/day10.aoc")?;
    let reader = io::BufReader::new(file);

    let mut mapped: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines(){
        let line = line?;
        mapped.push(line.chars().map(|x| x.to_digit(10).unwrap() as i32).collect());
    }


    let solution = part_1(&mapped);
    println!("Part 1 answer: {}", solution.0);
    println!("Part 2 answer: {}", solution.1);

    Ok(())

}
