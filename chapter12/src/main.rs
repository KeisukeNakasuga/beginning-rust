use std::mem::*;

fn main() {
    println!("12.1 ===================");
    // オブジェクトのサイズを調べる
    // 型のオブジェクトが占めるビット数を返却する
    println!("{} ", std::mem::size_of::<i32>());
    // オブジェクトのサイズをバイト数で返却する
    println!("{} ", std::mem::size_of_val(&12));

    println!("12.3 ===================");
    println!("{} {} {} {} {} {} {} {} {} {} {} {} {} {}",
             size_of::<i8>(), size_of::<u8>(), size_of::<i16>(), size_of::<u16>(),
             size_of::<i32>(), size_of::<u32>(), size_of::<i64>(), size_of::<u64>(),
             size_of::<i128>(), size_of::<u128>(), size_of::<f32>(), size_of::<f64>(),
             size_of::<bool>(), size_of::<char>());

    println!("12.4 ===================");
    // プリミティブ型の表現
    fn as_bytes<T>(o: &T) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                o as *const _ as *const u8,
                std::mem::size_of::<T>())
        }
    }

    // メモリに格納されたバイト列としての表現を出力
    println!("{:?}", as_bytes(&1i8));

    println!("12.5 ===================");
    // メモリ上のバイト位置

    // 仮想メモリアドレスを出力する
    let b1 = true;
    let b2 = true;
    let b3 = false;
    println!("{} {} {}",
             &b1 as *const bool as usize,
             &b2 as *const bool as usize,
             &b3 as *const bool as usize);

    // 16新数表記で出力
    println!("{:p} {:p} {:p}", &b1, &b2, &b3);

    println!("12.6 ===================");
    // 複合データ型のサイズ
    enum E1 { E1a, E1b }
    enum E2 { E2a, E2b(f64) }

    println!("{} {} {} {} {} {}",
             size_of_val(&[0i16; 80]),  // 16ビット(2バイト)の数を80個 → 80 * 2 = 160バイト
             size_of_val(&(0i16, 0i16)), // 16ビット整数と、64ビット整数1つのタプル → 8 * 2 + 6バイトのパディング
             size_of_val(&[(0i16, 0i64); 100]), // 16バイトのタプル * 100の配列 = 1600バイトのタプル
             size_of_val(&E1::E1a),  //
             size_of_val(&E2::E2a),  // バリアントが8バイトのものを含むenum → enum全体では16バイト どのバリアントの値も同じサイズを占めるから
             size_of_val(&vec![(0i16, 0i64); 100]));  // スタックに置かれるデータのサイズはコンパイル時に確定 → ベクターはヘッダのみスタックに置かれる → 24バイトの理由

    println!("12.7 ===================");
    // ベクターの割り当て
    // ベクター = スタックに割り当てる固定長ヘッダ + ヒープに割り当てる可変長バッファ
    let mut v = vec![0; 0];
    println!("{} {}", v.len(), v.capacity());
    v.push(11);
    println!("{} {}", v.len(), v.capacity());
    v.push(22);
    println!("{} {}", v.len(), v.capacity());
    v.push(33);
    println!("{} {}", v.len(), v.capacity());
    v.push(44);
    println!("{} {}", v.len(), v.capacity());
    v.push(55);
    println!("{} {}", v.len(), v.capacity());
}
