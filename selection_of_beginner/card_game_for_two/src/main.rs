use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input);
    stdin().read_line(&mut input);
    let mut stack:Vec<i32> = input
                            .split_whitespace()
                            .map(|x| x.parse::<i32>().unwrap())
                            .collect();
    let mut point_of_alice = 0;
    let mut point_of_bob = 0;
    stack.remove(0);
    stack.sort();
    loop {
        point_of_alice += match stack.pop() {Some(num) => num, None =>break,};
        point_of_bob += match stack.pop() {Some(num) => num, None =>break,};
    }
    println!("{}", point_of_alice - point_of_bob);
    // println!("alice{}", point_of_alice);
    // println!("bob{}",point_of_bob);
}
