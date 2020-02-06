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

    let mut input = String::new();
    for i in 0..stack[1] {
        stdin().read_line(&mut input);
    }
    let mut input: Vec<i32> = input.split_whitespace()
                                    .map(|x| x.parse::<i32>().unwrap())
                                    .collect();
    let mut damages: Vec<i32> = Vec::new();
    let mut costs: Vec<i32> = Vec::new();
    for i in 0..stack[1] {
        costs.push(input.pop().unwrap());
        damages.push(input.pop().unwrap());
    }

    let mut minimun_cost = 100000000;
    let mut remained_life = 0;
    let mut total_attack = 0;
    let mut cost_for_i = 0;
    for i  in 0..stack[1] {
        total_attack = stack[0] % damages[i as usize];
        cost_for_i = stack[0] / damages[i as usize] * costs[i as usize];
        println!("damage of {} =================================",damages[i as usize] );
        for j in 0..stack[1] {
            println!("times: {}, danage: {}",j , damages[j as usize]);
            remained_life = total_attack;
            println!("before remain:{}",remained_life );
            let mut cost = cost_for_i;
            println!("before cost{}, stack: {}, damages:{}, costs:{}",cost, stack[0], damages[j as usize], costs[j as usize] );
            while remained_life > 0 {
                remained_life -= damages[j as usize];
                cost += costs[j as usize];
            }
            println!("remained_life:{}", remained_life );
            println!("costs{}",cost );
            println!("====================");
            if minimun_cost > cost { minimun_cost = cost}
        }
    }
    println!("{}",minimun_cost );
}

// fn vector_sort(vec: &mut Vec<[i32; 2]>, index: &i32, value: &i32) {
//     let minimum = 0;
//     let mut new_vec: Vec<[i32; 2]> = Vec::new();
//     for v in vec {
//         if v[value] > minimum {}
//     }
// }

// use std::io::stdin;
// use std::collections::HashSet;

// fn main() {
//     let mut input = String::new();
//     stdin().read_line(&mut input).unwrap();
//     let times: i32 = match input.trim().parse() {Ok(num) => num, Err(_) => return,};
//     let mut input = String::new();
//     for i in 0..times {
//         stdin().read_line(&mut input).unwrap();
//     }
//     let vec: HashSet<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
//     println!("{}", vec.len());
// }