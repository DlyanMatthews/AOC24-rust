use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let input = read_to_string(
        "./src/inputs/in3.txt").unwrap();
    let reg = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)")
        .unwrap();

    let ops: Vec<&str> = reg.find_iter(&input)
        .map(|s| s.as_str())
        .collect();

    let mut tot: i64 = 0;

    for i in &ops {
        let temp: Vec<&str> = i
            .split(",").collect();
        let temp2 = vec![
            temp[0][4..].parse::<i32>().unwrap(), 
            temp[1][..temp[1].len()-1].parse::<i32>().unwrap()
        ];
        tot += (temp2[0] * temp2[1]) as i64;        
    }

    println!("part 1: {:?}", tot);

    let reg_on = Regex::new(r"do\(\)").unwrap();
    let reg_off = Regex::new(r"don\'t\(\)").unwrap();

    let mut on: Vec<usize> = reg_on.find_iter(&input)
        .map(|n| n.start()).collect();
    let mut off: Vec<usize> = reg_off.find_iter(&input)
        .map(|n| n.start()).collect();
    let mut ops2: Vec<usize> = reg.find_iter(&input)
        .map(|n| n.start()).collect();

    tot = 0;
    on.push(0); off.push(0); ops2.push(0);
    let (mut in1, mut in2, mut in3) = (0,0,0);
    let mut flag = true;
    for i in 0..input.len() {
        if i == on[in1] {
            in1 += 1;
            flag = true;
        } else if i == off[in2] {
            in2 += 1;
            flag = false;
        } if i == ops2[in3] {
            if flag {
                let temp: Vec<&str> = ops[in3]
                    .split(",").collect();
                let temp2 = vec![
                    temp[0][4..].parse::<i32>().unwrap(), 
                    temp[1][..temp[1].len()-1].parse::<i32>().unwrap()
                ];
                tot += (temp2[0] * temp2[1]) as i64;
            }
            in3 += 1;
        }
    }

    println!("part 2: {:?}", tot);

    
}