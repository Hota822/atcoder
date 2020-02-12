pub fn main() {
    use std::collections::BinaryHeap;
    use std::collections::VecDeque;
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let nda = s
        .split_whitespace()
        .map(|w| w.parse().unwrap())
        .collect::<Vec<i64>>();
    let n = nda[0];
    let d = nda[1];
    let a = nda[2];
    let mut heap = BinaryHeap::with_capacity(n as usize);
    for _ in 0..n {
        s.clear();
        std::io::stdin().read_line(&mut s).unwrap();
        let xh = s
            .split_whitespace()
            .map(|w| w.parse().unwrap())
            .collect::<Vec<i64>>();
        heap.push((xh[0], xh[1]));
    }
    let mut count = 0;
    let mut damage = 0;
    let mut queue: VecDeque<(i64, i64)> = VecDeque::new();
    while !heap.is_empty() {
        // 使用済みの爆弾の影響がないとき || モンスターの位置が前に使った爆弾の範囲に入っていた時
        if queue.is_empty() || heap.peek().unwrap().0 >= queue.front().unwrap().0 {
            // ヒープからモンスターを取得
            let (x, h) = heap.pop().unwrap();
            // モンスターが生きている場合
            if h > damage {
                // 爆弾の使用回数
                let q = (h - damage + a - 1) / a;
                // 累計の爆弾の使用回数
                count += q;
                // 累積ダメージを計算
                damage += q * a;
                // 今回使用した爆弾の影響範囲とダメージをキューに保存
                queue.push_back((x - 2 * d, q * a));
            }
        } else {
            // モンスターの位置が爆弾の範囲外の時
            let (_, h) = queue.pop_front().unwrap();
            // 累積ダメージを減らす
            damage -= h;
        }
    }
    println!("{}", count);
}