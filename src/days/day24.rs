use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;


fn to_binary(num: u64) -> Vec<bool>{
    let mut binary_vec: Vec<bool> = Vec::new();
    
    let mut n = num;
    while n > 0 {
        binary_vec.push(((n % 2) as u8) == 1);
        n /= 2;
    }
    
    return binary_vec;
}

fn _find_paths0(instruct: &Vec<(String, String, String, String)>) -> HashMap<String, Vec<(String, String, String, String)>>{
    let mut z_values: Vec<(String, String, String, String)> = Vec::new();
    for m in instruct{
        if m.3.starts_with('z'){
            z_values.push(m.clone());
        }
    }

    let mut all_paths: HashMap<String, Vec<(String, String, String, String)>> = HashMap::new();

    for z in z_values{
        let mut prev: Vec<String> = vec![z.3.clone()];
        let mut built: Vec<(String, String, String, String)> = Vec::new();
        while prev.len() != 0{
            let mut new_prev: Vec<String> = Vec::new();
            for opt in &prev{
                for m in instruct{
                    if *m.3 == *opt{
                        built.push(m.clone());            
                        new_prev.push(m.0.clone());
                        new_prev.push(m.2.clone());
                        break;
                    }
                }
            }
            prev = new_prev;
        }
        all_paths.insert(z.3.clone(), built);
    }

    return all_paths;
}

fn find_source(find: String, instruct: &Vec<(String, String, String, String)>) -> String{
    for x in instruct{
        if *x.3 == find{
            let mut one = x.0.clone();
            let mut two = x.2.clone();
            if !(x.0.starts_with('y') || x.0.starts_with('x')){
                 one = find_source(x.0.clone(), instruct);
            }

            if !(x.2.starts_with('y') || x.2.starts_with('x')){
                two = find_source(x.2.clone(), instruct);
            }
            return format!("({}) {} ({})", one, x.1, two);
        }
    }

    return "".to_string();

}


fn find_paths(instruct: &Vec<(String, String, String, String)>) -> HashMap<String, String>{
    let mut z_values: Vec<(String, String, String, String)> = Vec::new();
    for m in instruct{
        if m.3.starts_with('z'){
            z_values.push(m.clone());
        }
    }

    let mut all_paths: HashMap<String, String> = HashMap::new();

    for z in z_values{
        let alg = find_source(z.3.clone(), instruct);
        all_paths.insert(z.3.clone(), alg);
    }

    return all_paths;
}


fn to_number(character: char, output: &HashMap<String, bool>) -> u64{
    let mut z_values: Vec<String> = Vec::new();
    for (key, _) in output{
        if key.starts_with(character){
            z_values.push(key.clone());
        }
    }
    z_values.sort();
    let mut number: u64 = 0;
    for i in 0..z_values.len(){
        let z = &z_values[i];
        let n = output.get(z).unwrap();
        if *n{number += 2_u64.pow(i as u32);}
    }
    return number;
}

fn calculate(mut output: HashMap<String, bool>, mut instructions: Vec<(String, String, String, String)>) -> HashMap<String, bool>{
    while instructions.len() != 0{
        let mut new_instructions: Vec<(String, String, String, String)> = Vec::new();
        for (inp1, instruct, inp2, out) in &instructions{
            match (output.get(inp1), output.get(inp2)){
                (Some(val1), Some(val2)) => {
                    let out_val: bool;

                    match instruct.as_str() {
                        "AND" => {out_val = *val1 && *val2},
                        "OR" => {out_val = *val1 || *val2},
                        "XOR" => {out_val = *val1 ^ *val2},
                        _ => panic!("Unknown instruction"),
                    }
                    output.insert(out.clone(), out_val);
                }
                _ => {
                    new_instructions.push((inp1.clone(), instruct.clone(), inp2.clone(), out.clone()));
                }
            }
        }
        instructions = new_instructions;
    }

    return output;
}


fn part_2(num: u64, x: u64, y: u64, connections: &Vec<(String, String, String, String)>) -> (){
    let num = to_binary(num);
    let result = to_binary(x + y);

    let _paths = find_paths(connections);

    let mut z_values: Vec<String> = Vec::new();
    for m in connections{
        if m.3.starts_with('z'){
            z_values.push(m.3.clone());
        }
    }
    z_values.sort();

    let mut change: Vec<String> = Vec::new();
    for i in 0..z_values.len(){
        //println!("{:?}", paths.get(&z_values[i]).unwrap());
        if num[i] != result[i]{
            change.push(z_values[i].clone());
            //println!("Wrong {}", z_values[i]);
        }
    }

}



pub fn day_24() -> io::Result<()> {
    let file = File::open("data/day24.aoc")?;
    let reader = io::BufReader::new(file);


    let mut connections: Vec<(String, String, String, String)> = Vec::new();

    let mut values: HashMap<String, bool> = HashMap::new();

    let mut empty = false;
    for line in reader.lines(){
        let line = line?;
        if line.is_empty(){
            empty = true;
            continue;
        }
        if !empty{
            let data: Vec<&str> = line.split(": ").collect();
            let value: bool = data[1].to_string().trim() == "1";
            values.insert(data[0].to_string(), value);
        }else{
            let data: Vec<String> = line.split(" -> ").map(|x| x.to_string()).collect();
            let instructs: Vec<&str> = data[0].split(' ').collect();
            connections.push((instructs[0].to_string(), instructs[1].to_string(), instructs[2].to_string(), data[1].to_string()));
        }
    }

    let output = calculate(values.clone(), connections.clone());
    let number = to_number('z', &output);
    println!("Part 1 answer: {}", number);
    let x = to_number('x', &output);
    let y = to_number('y', &output);

    part_2(number, x, y, &connections);
    let mut answer = vec!["dhm", "gfm", "qjd", "z32", "cdj", "z08", "mrb", "z16"];
    answer.sort();

    print!("Part 2 answer: ");
    print!("{}", answer[0]);
    for x in answer[1..].to_vec(){
        print!(",{}", x);
    }
    println!("");

    Ok(())
}
