use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;
use std::collections::HashMap;


fn get_new(cur: &Vec<String>, connections: &HashMap<String, HashSet<String>>) -> Vec<String>{
    let mut out: Vec<String> = Vec::new();
    let options = connections.get(&cur[0]).unwrap();
    for possible in options{
        if *possible < cur[cur.len() - 1]{continue;}
        let mut poss = true;
        for c in cur{
            if !connections.get(c).unwrap().contains(possible){
                poss = false;
                break;
            }
        }
        if poss{
            out.push(possible.clone());
        }
    }

    return out;
}


pub fn day_23() -> io::Result<()> {
    let file = File::open("data/day23.aoc")?;
    let reader = io::BufReader::new(file);

    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();
    let mut all_computers: HashSet<String> = HashSet::new();

    for line in reader.lines(){
        let line = line?;
        let splitted: Vec<String> = line.splitn(2, '-').map(|s| s.to_string()).collect();
        let (a, b) = (splitted[0].clone(), splitted[1].clone());
        all_computers.insert(a.clone());
        all_computers.insert(b.clone());
        connections.entry(a.clone()).or_insert(HashSet::new()).insert(b.clone());
        connections.entry(b.clone()).or_insert(HashSet::new()).insert(a.clone());
    }

    let mut all_ts: HashSet<String> = HashSet::new();

    for computer in &all_computers{
        if computer.starts_with('t'){
            all_ts.insert(computer.to_string());
        }
    }

    let mut options: HashSet<Vec<String>> = HashSet::new();
    for tt in all_ts{
        options.insert(vec![tt]);
    }
    options = HashSet::new();
    for tt in &all_computers{
        options.insert(vec![tt.clone()]); 
    }
    
    let mut size = 0;
    loop{
        let mut new_options: HashSet<Vec<String>> = HashSet::new();
        for opt in &options{
            let possible = get_new(opt, &connections);
            for poss in possible{
                let mut new = opt.clone();
                new.push(poss.clone());
                new_options.insert(new);
            }
        } 
        if options.len() == 1 && new_options.len() == 0{
            let element = options.iter().next().unwrap();
            print!("Part 2 answer:");
            print!("{}", element[0]);
            for x in element[1..].to_vec(){
                print!(",{}", x);
            }
            println!("");
            break;
        }
        options = new_options;
        
        if options.len() != 0{

        }
        if size == 1{
            let mut length = 0;
            for opt in &options{
                for x in opt{
                    if x.starts_with('t'){
                        length += 1;
                        break;
                    }
                }
            }
            println!("Part 1 answer: {}", length);
        }
        size += 1;
    }

    Ok(())
}
