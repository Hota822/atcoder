use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut stack = [0; 2]; // 0: N, 1: Y
    for (i, j) in input.split_whitespace().enumerate() {
        stack[i] = j.parse().unwrap();
    }
    stack[1] /= 1000;
    // x + y + z = N
    // 10x + 5y + z = Y
    let (mut x, mut y) = (0, 0);
    for z in 0..(stack[0] + 1) {
        // print!("z:{} ", z);
        x = (-5 * stack[0] + stack[1] + 4 * z) / 5;
        y = stack[0] -z -x;
        // println!("x: {}, y: {}", x, y);
        // println!("{}",10 * x + 5 * y + z);
        if 10 * x + 5 * y + z == stack[1] {
            if x < 0 || y < 0 || z < 0 {continue;}
            println!("{} {} {}", x, y, z);
            return;
        }
    }
    println!("-1 -1 -1");
}
