use std::io::stdin;
fn main() {
    let mut input = String::new();
    let mut stack = Vec::new();
    stdin().read_line(&mut input).expect("s");
    stack.push(match input.trim().parse::<i32>() {Ok(num) => num,Err(_) => return,});
    stdin().read_line(&mut input).expect("s");
    for chr in input.split_whitespace() {
        stack.push(match chr.trim().parse::<i32>() {Ok(num) => num,Err(_) => return,})
    }
    stdin().read_line(&mut input).expect("s");
    let mut sum: i32 =  0;
    for i in stack {
        sum += i;
    }
    println!("{}",input);
}
