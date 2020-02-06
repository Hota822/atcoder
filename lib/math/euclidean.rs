

fn main() {
    let mut input  = String::new();
    std::io::stdin().read_line(&mut input);
    let mut stack = [0; 2];
    for (i, j ) in input.split_whitespace().enumerate() {
        stack[i] = j.parse::<i32>().unwrap();
    }

    println!("{}", euclid(&stack[0], &stack[1]));

}

fn divisor() {
    println!("Hello, world!");
}

fn multiple() {}

fn euclid(ref_x: &i32, ref_y: &i32) -> i32 {
    let mut x: i32 = ref_x.clone();
    let mut y: i32 = ref_y.clone();
    let mut remainder: i32 = x % y;
    while remainder != 0 {
        x = y;
        y = remainder;
        remainder = x % y;
    }
    y
}


//　ユークリッドの互除法の使用方法
    // 8177 / 3315 = 2 ... 1547
    // 3315 / 1547 = 2 ... 221
    // 1547 / 221  = 7 ... 221
    // 221  / 221  = 1 ... 0 >>終了
    //         ↑　これが最大公約数

// ユークリッドの互除法の証明
    // x = yq + r  --(1)
    // m をx, yの最大公約数とする --(2)
    // n をy, rの最大公約数とする --(3)
    // (2)を数式化　x = mx', y = my' --(4)
    // (1, 4)より mx' = my'q + r
    // m(x' - y'q) = r >> m は r の約数  --(5)
    // (2, 5)より n >= m --(6)
    // 同様に(3)を数式化　y = ny'', r = nr' --(7)
    // (1, 7)より x = nqy'' + nr'
    // x = n(qy'' + r') >> n はxの約数 --(8)
    // (3, 8)より m >= n --(9)
    // (6, 9)より　m = n