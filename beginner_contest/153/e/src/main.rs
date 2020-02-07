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

    let hp = stack[0]; // columns
    let types = stack[1]; // rows
    let mut dp_table:Vec<i32> = vec![100000000; (hp + 1) as usize];
    dp_table[0] = 0;

    // println!("hp: {}, types:{}, len:{}",hp, types, dp_table.len() );
    for t in 1..(types + 1) {
        let t: usize = t as usize;
        for h in 1..(hp + 1) {
            // println!("in for " );
            // println!("  t:{}, h:{}",t, h );
            // println!("  costs[t -1]:{}",costs[t -1 as usize]);
            // println!("  dp_table[h]:{}",dp_table[h as usize] );
            // for i in &dp_table {
            //     // print!("{}, ",i );
            // }
            

            if h <= damages[t - 1] {
                let h: usize = h as usize;
                //1回で大丈夫なとき
                if costs[t -1] < dp_table[h] {
                    dp_table[h] = costs[t -1];
                }
                // println!("  if");
                // println!("    dp_table: {}", dp_table[h as usize]);
                // println!("    damages:{}",damages[t - 1]  );
                // println!("    cost: {}", costs[t - 1] );
            } else {
                // tableの現在の最小値と、攻撃可能な時の最小値を比較
                let h: usize = h as usize;
                // println!("  ok");
                // println!("    damages:{}",damages[t - 1]  );
                // println!("    dp_before: {}", dp_table[h - damages[t - 1] as usize]);
                // println!("    cost: {}", costs[t - 1] );
                if dp_table[h] > ( dp_table[h - damages[t - 1] as usize] + costs[t - 1] ) {
                    // println!("ok");
                    dp_table[h] = dp_table[h - damages[t - 1] as usize] + costs[t - 1];
                }
            }
        }
        if t == 2 {break;}
    }
    println!("{}",dp_table.pop().unwrap());
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