fn main() {
    println!("16.5 ===================");
    // ベクター、配列、スライスのイテレータ
    // ベクター
    let vec_iterator = vec![10, 20, 30].into_iter();
    for item in vec_iterator {
        let j = item;
        print!("{} ", j + 1);
    }

    for item in vec![10, 20, 30].into_iter() {
        print!("{} ", item + 1);
    }

    println!();

    // 配列
    let array_iterator = [10, 20, 30].into_iter();
    for item in array_iterator {
        let j = item;
        print!("{} ", j + 1);
    }

    println!();

    // スライス
    let slice_iterator = [40, 50, 60][0..2].into_iter();
    for item in slice_iterator {
        let j = item;
        print!("{} ", *j + 1);
    }

    println!();

    // itemを変更する
    let v = vec![10, 20, 30];
    for mut item in v.into_iter() {
        item += 1;
        print!("{} ", item);
    }

    println!();

    let vv = vec![10, 20, 30];
    for item_ref in vv.iter() {
        print!("{} ", *item_ref + 1);
    }
    println!("{:?}", vv);

    println!();

    println!("16.8 ===================");
    let slice1 = &[3, 4, 5];
    let slice2 = &[7, 8];
    let mut iterator = slice1.iter();
    for item_ref in iterator {
        print!("{} ", *item_ref);
    }
    print!("; ");
    iterator = slice2.iter();
    for item_ref in iterator {
        print!("{} ", *item_ref);
    }

    println!();

    // 変更できるイテレータ~
    let mut vvv = vec![3, 4, 5];
    let iterator2 = vvv.iter_mut();
    for mut_item_ref in iterator2 {
        *mut_item_ref += 1;
    }
    println!("{:?}", vvv);

    println!("16.9 ===================");
    // イテレーターの略記法
    for item in vec![10, 20, 30].into_iter() {
        print!("{} ", item + 1);
    }

    println!();

    // 上記と同じ動きをする
    for item in &vec![10, 20, 30] {
        print!("{} ", item + 1);
    }

    println!();

    for item in &vec![10, 20, 30] {
        print!("{} ", *item + 1);
    }

    println!("16.11 ===================");
    // filter
    let arr = [66, -8, 43, 19, 0, -3];
    for n in arr.into_iter().filter(|x_ref| *x_ref < 0) {
        print!("{} ", n);
    }

    println!("16.12 ===================");
    // map
    let arrr = [66, -8, 43, 19, 0, -3];
    for n in arrr.into_iter().map(|x| x * 2) {
        print!("{} ", n);
    }

    println!("16.13 ===================");
    // enumerate
    let arrrr = ['a', 'b', 'c'];
    for (index, ch) in arrrr.into_iter().enumerate() {
        print!("{} {} ", index, ch);
    }

    println!("16.14 ===================");
    // any どれか
    let s = "Hello, world!";
    let ch = 'R';
    println!(
        "\"{}\" {} '{}'.",
        s,
        if s.chars().any(|c| c == ch) {
            "contains"
        } else {
            "does not contain"
        },
        ch
    );

    println!("16.15 ===================");
    // all すべて
    println!(
        "{} ",
        [45, 8, 2, 6].into_iter().all(|n: i32| -> bool { n > 0 })
    );
    println!(
        "{} ",
        [45, 8, -2, 6].into_iter().all(|n: i32| -> bool { n > 0 })
    );

    println!("16.16 ===================");
    // count
    let s = "ees";
    println!("{} {}", s.chars().count(), s.len());

    println!("16.17 ===================");
    // sum
    println!("{}", [45, 8, -2, 6].into_iter().sum::<i32>());
    let sum: i32 = [45, 8, -2, 6].into_iter().sum();
    println!("{}", sum);

    println!("16.18 ===================");
    // min/max
    let arrrr = ["hello", "world", "brave", "new"];
    match arrrr.into_iter().min() {
        Some(n) => print!("{}", n),
        _ => (),
    }

    println!("16.19 ===================");
    let arrrrr = [36, 1, 15, 9, 4];
    let v = arrrrr.into_iter().collect::<Vec<i32>>();
    println!("{:?}", v);

    println!("16.20 ===================");
    // イテレータチェイン
    let arrrrrr = [66, -8, 43, 19, 0, -31];
    let mut v = vec![];
    for i in 0..arrrrrr.len() {
        if arrrrrr[i] > 0 {
            v.push(arrrrrr[i] * 2);
        }
    }
    println!("{:?}", v);

    let vv = arrrrrr
        .into_iter()
        .filter(|x| *x > 0)
        .map(|x| x * 2)
        .collect::<Vec<_>>();
    println!("{:?}", vv);
}
