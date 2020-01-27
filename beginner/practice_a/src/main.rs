use std::io::stdin;
fn main() {
    let mut stack = [0; 3];
    let mut input = String::new();
    {
        let mut input = String::new();
        for _i in 0..2 {
            stdin().read_line(&mut input);
        }
        for (i, chr) in input.split_whitespace().enumerate() {
            stack[i] = match chr.trim().parse::<i32>() {Ok(num) => num,Err(_) => return,}
        }
    }
    stdin().read_line(&mut input);
    println!("{} {}",stack[0] + stack[1] + stack[2], input);
}
