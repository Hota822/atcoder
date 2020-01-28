use std::io::stdin;

fn main() {
    let mut input = String::new();
    //let mut stack = vec![String::new(); 2];
    stdin().read_line(&mut input);
    let mut i = 0;
    for chr in input.chars() {
        if chr == '1'{i += 1;}
    }
    println!("{}", i);
}
