use std::io::stdin;
fn main() {
    let mut input = String::new();
    let mut stack = [0; 2];
    stdin().read_line(&mut input);
    for (i, chr) in input.split_whitespace().enumerate() {
        stack[i] = match chr.parse() {Ok(num) => num, Err(_) => return,};
    }
    if (stack[0] * stack[1]) % 2 > 0 {
        println!("Odd");
        return;
    }
    println!("Even");
}
