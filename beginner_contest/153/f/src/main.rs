use std::io::stdin;

fn main() {
    let vec = get_input_by_vec();
    // 0: N 体のモンスター
    // 1: D 爆弾の範囲
    // 2: A 爆弾のダメージ
    let (mut each_hp_and_location, vector_size) = get_input_by_loop(vec[0]);
    let mut monster_location_and_hp = vec![0; vector_size + 1];
    let (mut location, mut hp) = (0, 0);
    let mut locations = Vec::new();
    for i in 0..vec[0] {
        // println!("{}",i );
        hp = each_hp_and_location.pop().unwrap();
        location = each_hp_and_location.pop().unwrap();
        // println!("location: {}, hp: {}",location, hp );
        monster_location_and_hp[location as usize] = hp;
        locations.push(location);
    }

    // print!("[");
    // for i in &monster_location_and_hp {
    //     print!("{}, ",i );
    // }
    // println!("]");

    locations.sort();
    let mut used_bomb: i64 = 0;
    let mut remained_monster = vec[0];
    let bomb_range = vec[1] * 2;
    let damage = vec[2];
    let mut counter = 0;
    let mut using_bomb_position = vector_size; // 暫定値

    while  locations.len() > 0 {
        // counter += 1;
        // println!("count {}",counter );

        // モンスターの位置を取得
        let next_monster_location = locations.pop().unwrap();

        //if // HP==0(前のBombで死んだとき）の時はcontinue
        if monster_location_and_hp[next_monster_location as usize] <= 0 {continue;}

        // BOMBダメージ計算（対象のモンスター）
        // 累積（巻き添え）ダメージを初期化
        let mut accumulation_damage = 0;

        // BOM位置が範囲を外れた時はBOMの範囲で使用
        if next_monster_location - (bomb_range / 2) < 0 {using_bomb_position = 0}

        //累積カウントの計算（Temp）
        accumulation_damage = (monster_location_and_hp[next_monster_location as usize] + damage - 1) / damage;

        // 使用回数を計算
        used_bomb += accumulation_damage as i64;

        accumulation_damage *= damage;

        // println!("  accumulation:{}",accumulation_damage );
        // println!("  next_monster_location: {}", next_monster_location);
        // BOMBダメージ計算（対象のモンスター）
        for i in 0..(bomb_range + 1) {

            // 範囲外に出たら終了
            if next_monster_location < i {break;}

            // bom positionの変更
            let position = (next_monster_location -i) as usize;
            // println!("target: {}",position );

            // 巻き添えダメージの計算
            monster_location_and_hp[position] -= accumulation_damage;

            // 累積ダメージで、またはもともといない時は次のループに
            if monster_location_and_hp[position] <= 0 {continue;}
        }
    }
    println!("{}",used_bomb);
}


fn get_input_by_vec() -> Vec<i32> {
    let mut input  = String::new();
    std::io::stdin().read_line(&mut input);
    let mut vec: Vec<i32> = input.split_whitespace()
                            .map(|x| x.parse().unwrap())
                            .collect();
    vec
}

fn get_input_by_loop(loop_count: i32) -> (Vec<i32>, usize) {
    let mut input = String::new();
    let mut max = 0;
    let mut temp = 0;
    for i in 0..loop_count {
        std::io::stdin().read_line(&mut input);
    }
    (input.split_whitespace()
         .map(|x| {
             temp = x.parse::<i32>().unwrap();
             if temp > max {max = temp;}
             x.parse::<i32>().unwrap()
         })
         .collect()
    , max as usize)
}