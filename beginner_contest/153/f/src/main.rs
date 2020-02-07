// use std::io::stdin;

fn main() {
    let vec: Vec<i32> = get_input_by_vec();
    // 0: N 体のモンスター
    // 1: D 爆弾の範囲
    // 2: A 爆弾のダメージ
    let (mut each_hp_and_location, mut locations) = get_input_by_loop(vec[0]);

    for i in &locations {
        println!("locations :{}",i );
    }
    for v in &each_hp_and_location {
        println!("vec[0]:{}, vec[1]: {}",v[0], v[1] );
    }

    // for v in &each_hp_and_location {
    //     locations.push(&v[0]);
    // }

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
    let mut counter = 0;
    let mut _using_bomb_position = 0; // 暫定値

    // println!("location.len:{}",locations.len() );
    while  locations.len() > 0 {
        counter += 1;
        println!("count {}",counter );

        // モンスターの位置を取得
        let next_monster_location = locations.pop().unwrap();

        // 累積（巻き添え）ダメージを初期化
        let mut accumulation_damage = 0;

        // 対象モンスターのHPを取得
        {
            let mut hp: &i32 = &0;
            for i in 0..each_hp_and_location.len() {
                if each_hp_and_location[i][0] != next_monster_location {continue;}
                hp = &each_hp_and_location[i][1];
                println!("hp{}================i:{}",hp, i );
            }

            // HP<=0(前のBombで死んだとき）の時はcontinue
            if hp <= &0 {continue;}

            // BOMBダメージ計算（対象のモンスター）
            // BOM位置が範囲を外れた時はBOMの範囲で使用
            if next_monster_location - (bomb_range / 2) < 0 {_using_bomb_position = 0}

            // 爆発の影響範囲計算
            effect_range = (next_monster_location - bomb_range)..(next_monster_location + 1);

            //累積カウントの計算（Temp）
            
            accumulation_damage = (hp + damage - 1) / damage;
            println!("{:.5}", ((hp + damage -1) as f64) / damage as f64 );
            println!("accum {}",accumulation_damage );
            println!("hp: {}, damage{}", hp, damage);
        }

        // 使用回数を計算
        used_bomb += accumulation_damage as i64;

        // 累積ダメージの計算
        accumulation_damage *= damage;

        // println!("  accumulation:{}",accumulation_damage );
        // println!("  next_monster_location: {}", next_monster_location);
        // BOMBダメージ計算（対象のモンスター）
        for i in 0..each_hp_and_location.len() {
            // 範囲にいた時の処理
            match each_hp_and_location[i][0] {
                effect_range => {
                    // 巻き添えダメージの計算
                    each_hp_and_location[i][1] -=  accumulation_damage;
                    // 累積ダメージで、またはもともといない時は次のループに
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

fn get_input_by_loop(loop_count: i32) -> (Vec<Vec<i32>>, Vec<i32>) {
    let mut input = String::new();
    let mut vec: Vec<Vec<i32>> = Vec::new();
    let mut locations: Vec<i32> = Vec::new();
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
        locations.push(y);
        println!("{}", y)
    }
    // println!("locations.len(in function): {}",locations.len() );
    for i in &locations {
        println!("locations :{}",i );
    }
    for v in &vec {
        println!("vec[0]:{}, vec[1]: {}",v[0], v[1] );
    }
    (vec, locations)
}
