use demo::my_lib;


fn main() {
    let input = my_lib::read_lines("./src/inputs/in1.txt");
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    for i in input {
        my_lib::parse_pairs(&mut vec1, &mut vec2, &i);
    }
    vec1.sort();
    vec2.sort();
    let mut tot = 0;
    for i in 0..vec1.len() {
        tot += (vec1[i]-vec2[i]).abs();
    }
    println!("part 1: {}", tot);

    tot = 0;
    for i in vec1 {
        tot += i * (vec2
        .iter()
        .filter(|n| **n == i)
        .count()) as i32;
    }
    println!("part 2: {}",tot);
}
