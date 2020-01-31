use std::io::stdin;

fn main() {
    {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    }
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut stack: Vec<i32> = input.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    let mut count = 0;
    loop {
        let mut temp_stack = Vec::new();
        for i in &stack {
            if  i % 2 > 0 {
                println!("{}", count);
                return;
            }
            temp_stack.push(i / 2);
        }
        stack = temp_stack;
        count += 1;
    }
}
