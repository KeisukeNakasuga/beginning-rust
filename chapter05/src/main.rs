fn main() {
    // 5.1 配列
    println!("5.1 ====================");
    let x = ["English", "This", "sentence"];
    println!("{} {} {}", x[0], x[1], x[2]);
    println!("len: {}", x.len());

    println!("5.2 ====================");
    // 5.2 アトリビュート
    #[deny(unused_variables)]
    // let xx = 1;  コンパイルエラーを出す
    #[warn(unused_variables)]
    let y = 2;
    #[allow(unused_variables)]
    let z = 3;

    println!("5.4 ====================");
    let mut a = ["this", "is", "a", "sentence"];  // aの型は「要素が文字列で長さが4の配列」
    a[2] = "a nice";
    print!("{} {} {} {}.", a[0], a[1], a[2], a[3]);

    println!("5.5 ====================");
    // 配列のサイズを指定
    let mut arr = [4.; 5000];

    println!("5.7 ====================");
    // ベクター 実行時にサイズを定義できるオブジェクト
    let mut v = vec!["This", "is"];
    println!("{} {}. Length: {}", x[0], x[1], v.len());
    v.push("a");
    v.push("sentence");
    v[0] = "That";
    for i in 0..v.len() {
        println!(" {}", v[i]);
    }

    println!("5.9 ====================");
    let empty_arr = [""; 0];  // 要素が文字列の空配列

    println!("5.11 ====================");
    let mut a1 = vec![4, 56, -2];
    let a2 = vec![7, 81, 125000];
    println!("{:?} {:?}", a1, a2);
    a1 = a2;
    println!("{:?}", a1); // a2はprintできない a2は「移動」されたため 移動はコピーしてオリジナルを破棄したことを意味する
    a1[1] = 10;
    println!("{:?}", a1);
}
