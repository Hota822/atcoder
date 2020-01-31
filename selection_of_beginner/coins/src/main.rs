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
    let mut sum_500 = 0;
    for _i in 0..(stack[0] + 1) {
        if sum_500 > stack[3] {break;}
        if sum_500 == stack[3] {count+=1;break;}
        let mut sum_100 = sum_500;
        for _j in 0..(stack[1] + 1) {
            if sum_100 > stack[3] {break;}
            if sum_100 == stack[3] {count+=1;break;}
            let mut sum_50 = sum_100;
            for k in 0..(stack[2] + 1) {
                if sum_50 > stack[3] {break;}
                if sum_50 == stack[3] {count+=1;break;}
                sum_50 += 50
            }
            sum_100 += 100;
        }
        sum_500 += 500;
    }
    println!("{}", count)
}
