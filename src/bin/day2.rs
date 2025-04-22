use demo::my_lib;

fn main() {
    let lines = my_lib::read_lines("./src/inputs/in2.txt");
    let input = my_lib::parse_vec(lines);

    let mut tot = 0;
    for line in &input {
        if foo(line) {
            tot += 1;
        }
    }
    println!("part 1: {:?}", tot);

    let mut tot = 0;
    for line in &input {
        if footoo(line) {
            tot += 1;
        }
    }
    println!("part 2: {:?}", tot);
}

fn foo(inp: &Vec<i32>) -> bool {
    let mut temp = inp[0];
    if inp[1] > temp {
        for i in &inp[1..] {
            if (i - temp) < 1 || (i - temp) > 3 {
                return false;
            } else {
                temp = *i;
                continue;
            }
        }
        return true;
    } else if inp[1] < temp {
        for i in &inp[1..] {
            if (temp - i) < 1 || (temp - i) > 3 {
                return false;
            } else {
                temp = *i;
                continue;
            }
        }
        return true;
    } else {
        return false;
    }
}

fn footoo(inp: &Vec<i32>) -> bool {
    for i in 0..inp.len() {
        let mut newline = inp.clone();
        newline.remove(i);
        if foo(&newline) {return true;}
    }
    return false;
}