use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input);
    let mut stack = [0; 2];
    for (i, j) in input.split_whitespace().enumerate() {
        stack[i] = j.parse().unwrap()
    }
    let mut times = stack[0] / stack[1];
    if stack[0] % stack[1] != 0 && stack[1] != 1 {times+=1}
    if stack[0] < stack[1] { times = 1};
    println!("{}", times);
}
