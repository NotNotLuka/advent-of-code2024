use regex::Regex;
use std::io;
use std::fs;


pub fn day_3() -> io::Result<()> {
    let input = fs::read_to_string("data/day3.aoc")?;

    let pattern = r"mul\(\d{1,3},\d{1,3}\)";
    let re = Regex::new(pattern).unwrap();

    let mut to_calculate = Vec::new();
    for found in re.find_iter(&input){
        to_calculate.push(found);
    }


    let mut sum = 0;
    for mul in &to_calculate{
        let mul_str = &mul.as_str();
        let nums = &mul_str[4..mul_str.len()-1];
        let mut x = 1;
        for n in nums.split(','){
            if let Ok(val) = n.parse::<i32>() {
                x *= val;
            }
            else{
                panic!("Failed to parse number: {}", n);
            }
        }
        sum += x;
    }
    println!("Part 1 answer: {}", sum);



    let dos = Regex::new(r"do\(\)").unwrap();
    let donts = Regex::new(r"don\'t\(\)").unwrap();

    let mut offs = Vec::new();
    let mut ons = Vec::new();
    for found in donts.find_iter(&input){
        offs.push(found.start());
    }
    for found in dos.find_iter(&input){
        ons.push(found.start());
    }


    let mut on = true;
    let mut sum = 0;

    for mul in &to_calculate{
        let ind = &mul.start();
        let mul_str = mul.as_str();
        
        if !offs.is_empty() && !ons.is_empty() &&
           ind > offs.get(0).unwrap() && ind > ons.get(0).unwrap(){
            if offs.get(0).unwrap() < ons.get(0).unwrap() {on = true;}
            else {on = false;}
            offs.remove(0); ons.remove(0);
        }
        else if !offs.is_empty() && 
             ind > offs.get(0).unwrap(){
            on = false; offs.remove(0);
        }
        else if !ons.is_empty() && 
             ind > ons.get(0).unwrap(){
            on = true; ons.remove(0);
        }

        if !on {continue;}
        let nums = &mul_str[4..mul_str.len()-1];
        let mut x = 1;
        for n in nums.split(','){
            if let Ok(val) = n.parse::<i32>() {
                x *= val;
            }
            else{
                panic!("Failed to parse number: {}", n);
            }
        }
        sum += x;
    }

    println!("Part 2 answer: {}",sum);
    Ok(())
}


