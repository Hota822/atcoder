use std::io::stdin;
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let times: i32 = match input.trim().parse() {Ok(num) => num, Err(_) => return,};
    let mut input = String::new();
    for i in 0..times {
        stdin().read_line(&mut input).unwrap();
    }
    let vec: HashSet<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    println!("{}", vec.len());
}

