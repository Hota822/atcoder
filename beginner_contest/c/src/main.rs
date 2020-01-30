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
    if stack[0] < stack[1] {println!("0");return;}

    let mut vec: Vec<i64> = values.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();
    vec.sort();
    let count_vec = &vec[0..(vec.len() - stack[1])];
    let mut times = 0;
    for i in count_vec.iter() {
        times = times + i;
        // println!("vec: {}",i );
    }
    println!("{}", times);
}