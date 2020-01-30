use std::io::stdin;
fn main() {
    let mut times = 0;
    {
    let mut input = String::new();
    stdin().read_line(&mut input);
    times = input.trim().parse::<i32>().unwrap() ;
    }
    let mut stack: [i32; 3] = [0; 3];
    let mut stack_before:[i32; 3] = [0; 3];
    let mut diff = [0; 3];
    let mut failure_flag = false;
    let mut print_sentence = "Yes";
    let mut values = String::new();

    for n in 0..times {
        {
        let mut input = String::new();
        stdin().read_line(&mut input);
        values = input;
        }
        if failure_flag {continue;}
        for (i, j) in values.split_whitespace().enumerate() {
            // println!("i: {}, {}, input: {}", i, j.parse::<i32>().unwrap(), values);
            stack[i] = j.parse::<i32>().unwrap();
            diff [i] = stack[i] - stack_before[i];
        }

        // println!("stack: {}, {}, {}", stack[0], stack[1], stack[2]);
        if !(diff[0].abs() % 2 == (diff[1] + diff[2]).abs() % 2)
            || stack[0] < stack[1] + stack[2]
        {
            print_sentence = "No";
            // println!("No, diff: {}, {}, {}", diff[0], diff[1], diff[2]);
            failure_flag = true;
        }
        stack_before = stack;
        // println!("ok this time {}", stack[0]);
    }
    println!("{}", print_sentence);
}
