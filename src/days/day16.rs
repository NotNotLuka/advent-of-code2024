use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::cmp::Reverse;


#[derive(Debug, Hash, Eq, Ord, PartialOrd, PartialEq, Clone)]
struct Node {
    x: i64,
    y: i64,
    dir: i64,
}


fn djikstra(start_node: Node, mut scores: HashMap<Node, i64>, all_moves: &HashMap<Node, Vec<(Node, i64)>>) -> HashMap<Node, i64>{

    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(Reverse((0, start_node.clone())));
    scores.insert(start_node, 0);

    while let Some(Reverse((score, node))) = priority_queue.pop(){
        if let Some(move_to_nodes) = all_moves.get(&node){
            for (move_to, add_score) in move_to_nodes{
                let cur_score = scores.get(move_to).unwrap();
                if score + add_score < *cur_score{
                    scores.insert(move_to.clone(), score + add_score);
                    priority_queue.push(Reverse((score + add_score, move_to.clone())));
                }
            }
        }
    }
    return scores;
}





pub fn day_16() -> io::Result<()> {
    let file = File::open("data/day16.aoc")?;
    let reader = io::BufReader::new(file);

    let mut mapped: Vec<Vec<char>> = Vec::new();
    for line in reader.lines(){
        let line = line?;
        mapped.push(line.chars().collect());
    }
    let mut start: (i64, i64) = (-1, -1);
    let mut end: (i64, i64) = (-1, -1);
    for y in 0..mapped.len(){
        for x in 0..mapped[0].len(){
            if mapped[y][x] == 'E'{
                end = (x as i64, y as i64);
            }
            if mapped[y][x] == 'S'{
                start = (x as i64, y as i64);
            }
        }
    }
    mapped[start.1 as usize][start.0 as usize] = '.';
    mapped[end.1 as usize][end.0 as usize] = '.';

    let mut scores: HashMap<Node, i64> = HashMap::new();
    let mut all_moves: HashMap<Node, Vec<(Node, i64)>> = HashMap::new();
    for y in 0..mapped.len(){
        for x in 0..mapped[0].len(){
            if mapped[y][x] == '.'{
                let x = x as i64;
                let y = y as i64;
                let frst = Node{x: x, y: y, dir: 0};
                let snd = Node{x: x, y: y, dir: 1};
                let third = Node{x: x, y: y, dir: 2};
                let fourth = Node{x: x, y: y, dir: 3};
                all_moves.entry(frst.clone()).or_insert(Vec::new()).extend(vec![(snd.clone(), 1000), (fourth.clone(), 1000)]);
                all_moves.entry(snd.clone()).or_insert(Vec::new()).extend(vec![(frst.clone(), 1000), (third.clone(), 1000)]);
                all_moves.entry(third.clone()).or_insert(Vec::new()).extend(vec![(snd.clone(), 1000), (fourth.clone(), 1000)]);
                all_moves.entry(fourth.clone()).or_insert(Vec::new()).extend(vec![(frst.clone(), 1000), (third.clone(), 1000)]);
                if ((x + 1) as usize) < mapped[0].len() && mapped[y as usize][(x + 1) as usize] == '.'{
                    all_moves.entry(snd.clone()).or_insert(Vec::new()).push((Node{x: x+1, y: y, dir: 1}, 1));
                }
                if 0 <= x - 1 && mapped[y as usize][(x - 1) as usize] == '.'{
                    all_moves.entry(fourth.clone()).or_insert(Vec::new()).push((Node{x: x-1, y: y, dir: 3}, 1));
                }

                if ((y + 1) as usize) < mapped.len() && mapped[(y + 1) as usize][x as usize] == '.'{
                    all_moves.entry(frst.clone()).or_insert(Vec::new()).push((Node{x: x, y: y + 1, dir: 0}, 1));
                }
                if 0 <= y - 1 && mapped[(y - 1) as usize][x as usize] == '.'{
                    all_moves.entry(third.clone()).or_insert(Vec::new()).push((Node{x: x, y: y - 1, dir: 2}, 1));
                }

                scores.insert(frst, i64::MAX);
                scores.insert(snd, i64::MAX);
                scores.insert(third, i64::MAX);
                scores.insert(fourth, i64::MAX);
            }
        }
    }

    let start_node = Node{x: start.0, y: start.1, dir: 1};
    let og_scores = djikstra(start_node.clone(), scores.clone(), &all_moves);

    let mut n = i64::MAX;
    for dir in 0..4{
        let x = og_scores.get(&Node {x: end.0, y: end.1, dir: dir}).unwrap();
        if *x < n{
            n = *x;
        }
    }
    println!("Part 1 answer: {}", n);

    let mut all_busy_tiles: HashSet<(i64, i64)> = HashSet::new();
    all_busy_tiles.insert((end.0, end.1));

    let mut end_scores: Vec<HashMap<Node, i64>> = Vec::new();
    for dir in 0..4{
        let tmp_node = Node{x: end.0, y: end.1, dir: dir};
        let new_scores = djikstra(tmp_node.clone(), scores.clone(), &all_moves);
        end_scores.push(new_scores.clone()); 
    }



    for (node, _) in &scores{
        let a_to_node = og_scores.get(&node).unwrap();
        let mut node_to_b = i64::MAX;
        for dir in 0..4{
            let reverse_dir = match node.dir {
                2 => 0,
                3 => 1,
                0 => 2,
                1 => 3,
                _ => panic!("Invalid direction value"),
            };
            let x = end_scores[dir].get(&Node {x: node.x, y: node.y, dir: reverse_dir}).unwrap();
            if *x < node_to_b{
                node_to_b = *x;
            }
        }
        if a_to_node + node_to_b == n{
            all_busy_tiles.insert((node.x, node.y));
        }
    }

    println!("Part 2 answer: {}", all_busy_tiles.len());

    Ok(())
}
