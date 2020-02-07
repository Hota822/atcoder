use std::io::stdin;

fn main {}
// range of using i32
//     about 2 * 10^9 ～ -2 * 10^9
//     ( -2,147,483,648 ～ 2,147,483,647 )

// 使う前に check point の型を確認
fn get_input_by_stack() -> [i32; 2] { // check point
    let mut input  = String::new();
    std::io::stdin().read_line(&mut input);
    let mut stack: [i32; 2] = [0; 2]; // check point 
    for (i, j ) in input.split_whitespace().enumerate() {
        stack[i] = j.parse().unwrap();
    }
    stack
}
