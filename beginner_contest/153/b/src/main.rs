//===========================================================================
// review time code
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input);
    let mut vec = Vec::with_capacity(2);
    vec = input.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    input.clear();
    stdin().read_line(&mut input);
    let mut sum = 0;
    for i in input.split_whitespace() {
        sum += i.parse::<i32>().unwrap();
    }
    if sum >= vec[0] {
        println!("Yes");
    } else {
        println!("No");
    }

}


//===========================================================================
// review time code
// using return in loop, but execution time is increased
// fn main() {
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input);
//     let mut vec = Vec::with_capacity(2);
//     vec = input
//         .split_whitespace()
//         .map(|x| x.parse().unwrap())
//         .collect::<Vec<i32>>();
//     let mut data = Vec::with_capacity(vec[1] as usize);
//     input.clear();
//     std::io::stdin().read_line(&mut input);
//     data = input
//         .split_whitespace()
//         .map(|x| x.parse().unwrap())
//         .collect::<Vec<i32>>();
//     let mut sum = 0;
//     let length = data.len() / 8;
//     // let remainder = data.len() % 4;
//     for i in 0..7 {
//         for i in 0..length {
//             sum += data.pop().unwrap();
//         }
//         if sum >= vec[0] {
//             println!("Yes");
//             return;
//         }
//     }
//     while !data.is_empty() {
//         sum += data.pop().unwrap();
//     }
//     if sum >= vec[0] {
//         println!("Yes");
//         return;
//     }
//     println!("No");
// }

// ===========================================================================
// first time code

// use std::io::stdin;

// fn main() {
//     let mut values = String::new();
//     {
//         let mut input = String::new();
//         stdin().read_line(&mut input);
//         values = input;
//     }
//     let mut stack = [0; 2];
//     for (i, j) in values.split_whitespace().enumerate() {
//         stack[i] = j.parse().unwrap()
//     }
//     {
//         let mut input = String::new();
//         stdin().read_line(&mut input);
//         values = input;
//     }
//     let mut sum = 0;
//     for i in values.split_whitespace() {
//         sum += i.parse::<i32>().unwrap();
//         println!("{}",i );
//     }
//     if sum >= stack[0] {
//         println!("Yes");
//         return
//     }
//     println!("No");
// }