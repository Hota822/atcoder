use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input);
    let mut vec = Vec::with_capacity(2);

    // create vec form splitted values
    vec = input
        .split_whitespace()
        .collect();

    // clear data in input
    input.clear();

    // get standard input
    stdin().read_line(&mut input);
    let mut num = Vec::with_capacity(2);

    // create num from splitted and parsed values
    num = input.split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<i32>>();

    // clear data in input;
    input.clear();
    stdin().read_line(&mut input);
    let mut target = Vec::with_capacity(1);
    target = input
        .split_whitespace()
        .collect();

    if target[0] == vec[0] {
        println!("{} {}",num[0] -1, num[1] );
    }

}
