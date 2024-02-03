fn main() {
    // 03-01
    let number = 12;
    let other_number = 53;
    print!("{}", number + other_number);

    // 可変変数
    let mut number_1 = 12;
    println!("{}", number_1);
    number_1 = 52;
    println!(" {}", number_1);

    // _
    // _は返却値を使い捨てるときに使用できる
    // 左辺にのみおける
    let _ = 13;
    // print!("{}", _); コンパイルエラー

    // 型推論
    // 次のような変数は無効
    // let number;

    // 3.10
    let mut n = 1;
    let n = 2;
    // 型が違う場合、再定義が必要
    let n = 3.14;

    // 3.12
    print!("{} {}", str::len("abcde"), "abcde".len());
}
