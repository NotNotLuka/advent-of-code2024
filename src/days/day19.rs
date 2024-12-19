use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;


fn count_option(flag: String, towels: &Vec<String>, mem: &mut HashMap<String, i64>) -> i64{
    if let Some(value) = mem.get(&flag){return *value;}
    
    let mut n = 0;
    for towel in towels{
        if flag.starts_with(towel){
            n += count_option(flag[towel.len()..].to_string(), towels, mem);
        }
    }
    mem.insert(flag, n);
    return n;

}

pub fn day_19() -> io::Result<()> {
    let file = File::open("data/day19.aoc")?;
    let reader = io::BufReader::new(file);

    let mut towels: Vec<String> = Vec::new();
    let mut flags: Vec<String> = Vec::new();

    let mut n = 0;
    for line in reader.lines(){
        let line = line?;
        n += 1;
        if n == 1{
            towels = line.split(',').map(|x| x.trim().to_owned()).collect();
            continue;
        }
        if n == 2{continue;}
        flags.push(line);
    }

    let mut n = 0;
    let mut m = 0;
    let mut mem: HashMap<String, i64> = HashMap::new();
    mem.insert("".to_string(), 1);
    for flag in &flags{
        let x = count_option(flag.to_string(), &towels, &mut mem);
        if x != 0{n += 1;}
        m += x;

    }
    println!("Part 1 answer: {}", n);
    println!("Part 2 answer: {}", m);

    Ok(())
}
