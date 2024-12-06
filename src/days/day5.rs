use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;
use std::collections::HashMap;


fn check_if_valid(update: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> bool {
        let mut illegal = HashSet::new();
        let mut legal = true;
        let reversed: Vec<i32> = update.iter().rev().cloned().collect();
        for x in &reversed{
            if illegal.contains(&x){
                legal = false;
                break;
            }
            if !&rules.contains_key(&x){continue;}
            for no_nums in &rules[&x]{
                illegal.insert(no_nums);
            }
        }
        return legal;
}


fn get_new(update: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
        let reversed: Vec<i32> = update.iter().rev().cloned().collect();
        for x in 0..reversed.len(){
            let y = reversed[x];
            let mut z = (x as i32) - 1;
            while 0 <= z{
                let candidate = reversed[z as usize];
                if !rules.contains_key(&candidate){z-=1; continue;}
                if rules[&candidate].contains(&y){
                    let mut new = update.clone();
                    new.remove(new.len() - x - 1);
                    new.insert(new.len() - (z as usize), y);
                    return get_new(&new, &rules);
                }
                z -= 1;
            }
        }
        return update.clone();
}


pub fn day_5() -> io::Result<()> {
    let file = File::open("data/day5.aoc")?;
    let reader = io::BufReader::new(file);

    let mut updates: Vec<Vec<i32>> = Vec::new();
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();

    let mut if_rules: bool = true;
    for line in reader.lines(){
        let line = line?;
        if line.trim().is_empty(){
            if_rules = false;
            continue;
        }
        if if_rules{
            let rule: Vec<i32> = line.split("|").map(|num| num.parse().unwrap()).collect();
            let (k, v) = (rule[0], rule[1]);
            match rules.get_mut(&k) {
              Some(vec) => vec.push(v),
              None => {rules.insert(k, vec![v]);}
            }
        }
        else{
            let nums: Vec<i32> = line.split(",")
              .map(|num| num.trim().parse().unwrap()).collect();
            updates.push(nums);
        }
    }

    let mut n = 0;
    let mut incorrect: Vec<Vec<i32>> = Vec::new();

    for update in updates{
        let legal: bool = check_if_valid(&update, &rules);
        if legal{
            let num = update[update.len() / 2];
            n += num;
        }
        else{
            incorrect.push(update);
        }
    }
    println!("Part 1 answer: {}", n);


    let mut n = 0;
    for inc in incorrect{

        let new = get_new(&inc, &rules);
        let num = new[new.len() / 2];
        n += num;
    }
    println!("Part 2 answer: {}", n);


    Ok(())
}
