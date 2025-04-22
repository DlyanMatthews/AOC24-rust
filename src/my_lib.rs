use std::fs::read_to_string;
use std::{thread::sleep, time::Duration};

//pub mod my_lib;

pub fn main() {}

pub fn countdown() {
    for n in 0..3 {
        println!("We're on in {}!", 3-n);
        sleep(Duration::from_secs(1));
    }
}

pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
    // Invis returns
}

pub fn parse_pairs(v1: &mut Vec<i32>, v2: &mut Vec<i32>, line: &str) {
    let nums: Vec<&str> = line.split_whitespace().collect();
    v1.push(nums[0].parse().expect("idk"));
    v2.push(nums[1].parse().expect("idk"));
}

pub fn parse_vec(inp: Vec<String>) -> Vec<Vec<i32>> {
    let mut oup: Vec<Vec<i32>> = Vec::new();
    for line in inp {
        oup.push
        ( 
            line
            .split_whitespace()
            .map
            (
                |n| n.parse::<i32>()
                .unwrap()
            )
            .collect()
        );
    }
    return oup;
}

pub fn check_bounds<T>(n: i32, v: &Vec<T>) -> bool {
    return n > 0 && n < v.len() as i32;
}