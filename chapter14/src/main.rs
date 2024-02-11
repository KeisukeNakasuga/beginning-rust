use std::mem::*;

fn main() {
    println!("14.5 ===================");
    // 静的文字列と動的文字列の変換
    let s1: String = "abc".to_string();
    let s2: &String = &s1; // 動的文字列への参照を新たな変数に代入
    let s3: &str = &s1; // &strは静的文字列への参照

    println!("14.6 ===================");
    // 文字列を結合する
    let vs = ["Hello", ", ", "world", "!"];
    let mut result = String::new();
    for s in vs {
        result.push_str(s);
    }
    println!("{}", result);

    let vs2 = ["Hello", ", ", "world", "!"];
    let mut result2 = String::new();
    for s in vs2 {
        result2 += s;
    }
    println!("{}", result2);
}
