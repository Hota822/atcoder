use std::io::stdin;

fn main() {
    let mut values = String::new();
    {
        let mut input = String::new();
        stdin().read_line(&mut input);
        values = input;
    }

    let mut values: i64 = values.trim().parse::<i64>().unwrap();
    let mut count: i64 = 1;
    let mut times: i64 = 0;
    loop {
        times += 1 * count;
        values /= 2;
        count *= 2;
        if values == 0 {
            break;
        }
    }
    println!("{}", times);
}