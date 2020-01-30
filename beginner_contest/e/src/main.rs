use std::io::stdin;

fn main() {
    let mut values = String::new();
    {
        let mut input = String::new();
        stdin().read_line(&mut input);
        values = input;
    }
    let mut stack = [0;2];
    for (i, j) in values.split_whitespace().enumerate() {
        stack[i] = j.parse::<i32>().unwrap();
    }

    let mut vec: Vec<[i32; 3]> = Vec::new();

    for k in 0..stack[1] {
        {
            let mut input = String::new();
            stdin().read_line(&mut input);
            values = input;
        }
        let pair = [0; 3];
        for (i, j) in values.split_whitespace().enumerate() {
            pair[i] = j.parse::<i32>().unwrap();
        }
        pair[2] = pair[0] / pair[1];
        vec.push(pair);
    }


    println!("{}", times);
}