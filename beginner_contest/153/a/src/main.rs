fn main() {
    use std::collections::VecDeque;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);
    let mut vec: Vec<i32> = Vec::with_capacity(2);
    vec = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();
    println!("{}", (vec[0] + vec[1] -1) / vec[1] );
}
