use std::io::stdin;

fn main() {
    let mut values = String::new();
    {
        let mut input = String::new();
        stdin().read_line(&mut input);
        values = input;
    }
    let mut stack = [0; 2];
    for (i, j) in values.split_whitespace().enumerate() {
        stack[i] = j.parse().unwrap()
    }
    {
        let mut input = String::new();
        stdin().read_line(&mut input);
        values = input;
    }
    let mut sum = 0;
    for i in values.split_whitespace() {
        sum += i.parse::<i32>().unwrap();
        println!("{}",i );
    }
    if sum >= stack[0] {
        println!("Yes");
        return
    }
    println!("No");
}