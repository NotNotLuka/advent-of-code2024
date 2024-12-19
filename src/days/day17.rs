use std::io::{self};


fn proper_execute(a_reg: u64) -> Vec<u64>{
    let mut a = a_reg;
    let mut output: Vec<u64> = Vec::new();
    while a != 0{
        let mut b = (a&7) ^ 5;
        b = (b^(a>>b)) ^ 6;
        a = a >> 3;
        output.push(b&7);
    }

    return output;
}


pub fn day_17() -> io::Result<()> {
    let instruct_line: Vec<u64> = vec![2,4,1,5,7,5,0,3,4,0,1,6,5,5,3,0];
    let output = proper_execute(46187030);
    println!("Part 1 answer: {:?}", output.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","));

    let mut solutions: Vec<u64> = Vec::new();
    let mut possible: Vec<u64> = Vec::new();
    let mut n_bits = 10;
    let first_max = 2_u64.pow(n_bits);
    for x in 1..first_max{
        let out = proper_execute(x);
        if instruct_line[0] == out[0] {
            possible.push(x);
        }
    }

    for n in 1..instruct_line.len(){
        let mut new_possible = Vec::new();
        for poss in &possible{
            for i0 in 0..2{
                for i1 in 0..2{
                    for i2 in 0..2{
                        let new = poss 
                            + i0 * 2_u64.pow(n_bits) 
                            + i1 * 2_u64.pow(n_bits + 1)
                            + i2 * 2_u64.pow(n_bits + 2);
                        let out = proper_execute(new);
                        
                        if out == instruct_line{
                            solutions.push(new);
                        }
                        else if n < out.len() && instruct_line[n] == out[n]{
                            new_possible.push(new);
                        }
                    }
                }
            }
        }
        possible = new_possible.clone();
        n_bits += 3;
    }
    solutions.sort();
    println!("Part 2 answer: {:?}", solutions[0]);

    Ok(())
}
