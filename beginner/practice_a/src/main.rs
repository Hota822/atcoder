fn main() {
    let mut stack = 0;
    let mut input = String::new();
    {
        let mut input = String::new();
        for _i in 0..2 {
            std::io::stdin().read_line(&mut input);
        }
        for chr in input.split_whitespace() {
            stack += match chr.parse::<i32>() {Ok(num) => num,Err(_) => return,}
        }
    }
    std::io::stdin().read_line(&mut input);
    println!("{} {}",stack, input.trim());
}
