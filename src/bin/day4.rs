use demo::my_lib::{self, check_bounds};

fn main() {
    let path = "./src/inputs/in4.txt";
    let input = my_lib::read_lines(path);
    let input2: Vec<Vec<char>> = input
        .iter()
        .map(|n| n.chars()
            .collect()
        ).collect();

    //every letter in array
    for y_pos in 0..input2.len() {
        for x_pos in 0..input2[y_pos].len() {
            //if its an X
            if input2[y_pos][x_pos] == 'X' {
                //check all 8 directions
                for i in 1..=8 {
                    let tx = x_pos as i32 + key(i).0;
                    let ty = y_pos as i32 + key(i).1;
                    if check_bounds(ty, &input2) 
                        && check_bounds(tx, &input2[y_pos]) {}
                }
            }
        }
    }

}

fn key(k: u8) -> (i32, i32) {
    match k {
        1 => (1,0),
        2 => (1,1),
        3 => (0,1),
        4 => (-1,1),
        5 => (-1,0),
        6 => (-1,-1),
        7 => (0,-1),
        8 => (1,-1),
        _ => panic!("invalid key")
    }
}