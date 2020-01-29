use std::io::stdin;

fn main() {
    let mut input = String::new();
    let mut stack = [0; 3];
    stdin().read_line(&mut input).unwrap();
    for (i, j) in input.split_whitespace().enumerate() {
        stack[i] = j.parse::<i32>().unwrap();
    }
    let mut sum = 0;
    if stack[0] == 10000 {
        if stack[1] < 2 {sum +=10000;};
        stack[0] -= 1;
    }
    // println!("stack0:{}", stack[0]);
    // println!("stack1:{}", stack[1]);
    // println!("sum:{}", sum);
    for i in 1..(stack[0]+1) {
        let mut temp = 0;
        let mut minus = 0;
        let mut calculated = 0;
        calculated = i / 10000;
        if calculated > 0 {
            temp += calculated;
            minus += 10000 * calculated;
        }
        calculated = i / 1000;
        if calculated > 0 {
            temp += calculated;
            minus += 1000 * calculated;
        }
        calculated = (i - minus) / 100;
        if calculated > 0 {
            temp += calculated;
            minus += 100 * calculated;
        }
        calculated = (i - minus) / 10;
        if calculated > 0 {
            temp += calculated;
            minus += 10 * calculated;
        }
        temp += i - minus;
        if temp >= stack[1] && temp <= stack[2] {
            sum += i;
            // println!("sum: {}, i: {}", sum, i);
        }
    }
    println!("{}", sum);
}
