fn main() {
    println!("15.1 ===================");
    for i in 0..12 {
        println!("{}", i);
    }

    let dozen = 0..12;
    for i in dozen {
        println!("{}", i);
    }

    // rangeオブジェクトが持っている情報
    let range: std::ops::Range<usize> = 3..8;
    println!(
        "{:?}, {}, {}, {}",
        range,
        range.start,
        range.end,
        range.len()
    );
    for i in range {
        println!("{}", i);
    }

    println!("15.2 ===================");
    // スライスの必要性
    fn min(arr: [i32; 8]) -> i32 {
        let mut minimum: i32 = arr[0];
        for i in 1..arr.len() {
            if arr[i] < minimum {
                minimum = arr[i];
            }
        }
        minimum
    }
    println!("{}", min([23, 17, 12, 16, 15, 28, 17, 30]));

    // スタックやキャッシュを消費したくない 引数をコピーさせない
    fn min2(arr: &[i32; 8]) -> i32 {
        let mut minimum: i32 = arr[0];
        for i in 1..arr.len() {
            if arr[i] < minimum {
                minimum = arr[i];
            }
        }
        minimum
    }
    println!("{}", min2(&[23, 17, 12, 16, 15, 28, 17, 30]));

    println!("15.3 ===================");
    fn min3(arr: &[i32]) -> i32 {
        let mut minimum = arr[0];
        for i in 1..arr.len() {
            if arr[i] < minimum {
                minimum = arr[i];
            }
        }
        minimum
    }
    println!("{}", min3(&[23, 17, 12, 16, 15, 28, 17, 30]));

    fn min4(arr: &[i32]) -> i32 {
        let mut minimum = arr[0];
        for i in 1..arr.len() {
            if arr[i] < minimum {
                minimum = arr[i];
            }
        }
        minimum
    }
    let arr = [23, 17, 12, 16, 15, 2];
    let range = 2..5;
    let slice_ref = &arr[range];
    println!("{}", min4(slice_ref));

    let arr2 = [55, 22, 33, 44, 66, 7, 8];
    let v = vec![55, 22, 33, 44, 66, 7, 8];
    let sr1 = &arr2[2..5];
    let sr2 = &v[2..5];
    println!("{:?} {:?} {:?} {:?}", sr1, sr2, &sr1[1..2], &sr1[1]);

    println!("15.7 ===================");
    // 半開区間のスライス
    let arr3 = [11, 22, 33, 44];
    let n = 2;
    let str1 = &arr3[0..n];
    let str2 = &arr3[n..arr3.len()];
    println!("{:?} {:?}", str1, str2);

    // 上限・下限を書かないスタイルもOK
    let str3 = &arr3[n..];
    let str4 = &arr3[..n];
    println!("{:?} {:?}", str3, str4);

    // イテレータと一緒にも書ける
    for i in 3.. {
        if i * i > 40 {
            break;
        }
        println!("{} ", i);
    }

    println!("15.8 ===================");
    let arr4 = [11, 12, 13, 14, 15];
    let r1 = 0..=3;
    println!("{:?}", &arr4[r1]);
}
