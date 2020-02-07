use std::io::stdin;
// 中身の型を変えたいときはcheck pointを変更
fn main {}
// range of using i32
//     about 2 * 10^9 ～ -2 * 10^9
//     ( -2,147,483,648 ～ 2,147,483,647 )


//=====================================================================================
// Single Line Input
//=====================================================================================
fn get_input_by_stack() -> [i32; 2] { // check point
    let mut input  = String::new();
    std::io::stdin().read_line(&mut input);
    let mut stack: [i32; 2] = [0; 2]; // check point
    for (i, j ) in input.split_whitespace().enumerate() {
        stack[i] = j.parse().unwrap();
    }
    stack
}

fn get_input_by_vec() -> Vec<i32> { //checkpoint
    let mut input  = String::new();
    std::io::stdin().read_line(&mut input);
    let mut vec: Vec<i32> = input.split_whitespace() // check point
                            .map(|x| x.parse().unwrap())
                            .collect();
    vec
}
//=======================================================================================
// multiple input
//=======================================================================================
fn get_input_by_loop(loop_count: i32) -> Vec<i32> {
    let mut input = String::new();
    for i in 0..loop_count {
        std::io::stdin().read_line(&mut input);
    }
    input.split_whitespace()
         .map(|x| x.parse::<i32>().unwrap())
         .collect()
}




