// use std::io::stdin;

fn main() {
    let vec: Vec<i32> = get_input_by_vec();
    // 0: N 体のモンスター
    // 1: D 爆弾の範囲
    // 2: A 爆弾のダメージ
    let mut each_hp_and_location = get_input_by_loop(vec[0]);
    let mut locations: Vec<&i32> = Vec::new();

    {
        for v in &each_hp_and_location {
            locations.push(&v[0]);
        }
    }

    // print!("[");
    // for i in &monster_location_and_hp {
    //     print!("{}, ",i );
    // }
    // println!("]");

    locations.sort();
    let mut used_bomb: i64 = 0;
    let bomb_range = vec[1] * 2;
    let mut effect_range = 0..0;
    let damage = vec[2];
    // let mut counter = 0;
    let mut _using_bomb_position = 0; // 暫定値

    while  locations.len() > 0 {
        // counter += 1;
        // println!("count {}",counter );

        // モンスターの位置を取得
        let next_monster_location = locations.pop().unwrap();

        // 対象モンスターのHPを取得
        let mut hp: &i32 = &0;
        {
            for v in &each_hp_and_location {
                if &v[0] != next_monster_location {continue;}
                hp = &v[1];
            }
        }

        // HP<=0(前のBombで死んだとき）の時はcontinue
        if hp <= &0 {continue;}

        // BOMBダメージ計算（対象のモンスター）
        // 累積（巻き添え）ダメージを初期化
        let mut _accumulation_damage = 0;

        // BOM位置が範囲を外れた時はBOMの範囲で使用
        if next_monster_location - (bomb_range / 2) < 0 {_using_bomb_position = 0}

        // 爆発の影響範囲計算
        effect_range = (next_monster_location - bomb_range)..(next_monster_location + 1);

        //累積カウントの計算（Temp）
        _accumulation_damage = (hp + damage - 1) / damage;

        // 使用回数を計算
        used_bomb += _accumulation_damage as i64;

        // 累積ダメージの計算
        _accumulation_damage *= damage;

        // println!("  accumulation:{}",accumulation_damage );
        // println!("  next_monster_location: {}", next_monster_location);
        // BOMBダメージ計算（対象のモンスター）
        {
            for mut v in &mut each_hp_and_location {
                // 範囲にいた時の処理
                match v[0] {
                    effect_range => {
                        // 巻き添えダメージの計算
                        v[1] -= _accumulation_damage;
                        // 累積ダメージで、またはもともといない時は次のループに
                    }
                }
            }
        }
    }
    println!("{}",used_bomb);
}


fn get_input_by_vec() -> Vec<i32> {
    let mut input  = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut _vec: Vec<i32> = input.split_whitespace()
                            .map(|x| x.parse().unwrap())
                            .collect();
    _vec
}

fn get_input_by_loop(loop_count: i32) -> Vec<Vec<i32>> {
    let mut input = String::new();
    let mut max = 0;
    let mut vec: Vec<Vec<i32>> = Vec::new();
    println!("start get" );
    for _i in 0..loop_count {
        std::io::stdin().read_line(&mut input).unwrap();
    }
    let mut temp: Vec<i32> = input.split_whitespace()
         .map(|x| x.parse::<i32>().unwrap())
         .collect();

    let (mut x, mut y) = (0, 0);
    loop{
        x = match temp.pop() {
            Some(v)  => v,
            None => break
        };
        y = temp.pop().unwrap();
        vec.push(vec![y, x]);
    }
    vec
}
