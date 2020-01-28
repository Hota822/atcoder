use std::io::stdin;

fn main() {
    let mut input = String::new();
    for _i in 1..5 {
        stdin().read_line(&mut input).unwrap();
    }
    let mut stack = [0; 4];
    for (i, chr) in input.split_whitespace().enumerate() {
        stack[i] = chr.parse::<i32>().unwrap();
    }
    let mut count = 0;
    let mut _remains = 0;
    for i in 0..(stack[0] + 1) {
        _remains = stack[3] - i * 500;
        if _remains < 0 {break;}
        // println!("i=======================: {}, remains: {}", i, _remains);
        for _j in 0..(stack[1] + 1) {
            if _remains < 0 {break;}
            _remains -=  100;
            // println!("j: {}, remains: {}", j, _remains);
            if _remains < stack[2] * 50 {
                count += 1;
                // println!("countup");
                continue;
            }
        }
    }
    println!("{}", count);
}
