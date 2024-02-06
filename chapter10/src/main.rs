use std::fmt::format;

fn main() {
    println!("10.2 ===================");

    // 10.2
    fn f<T>(ch: char, num1: T, num2: T) -> T {
        if ch == 'a' { num1 }
        else { num2 }
    }
    let a: i16 = f::<i16>('a', 37, 41);
    let b: f64 = f::<f64>('b', 37.2, 41.1);
    println!("{} {}", a, b);

    println!("10.3 ===================");
    // 型推論を使うことで型パラメータなしで書ける
    let aa: i16 = f('a', 37, 41);
    let bb: f64 = f('b', 37.2, 41.1);
    println!("{} {}", aa, bb);

    println!("10.4 ===================");
    // ジェネリック構造体
    // C++でいうとクラステンプレート/構造体テンプレート
    struct S<T1, T2> {
        c: char,
        n1: T1,
        n2: T1,
        n3: T2,
    }
    let _s = S { c: 'a', n1: 34, n2: 782, n3: 0.02 };

    println!("10.7 ===================");
    // ジェネリックな列挙体
    enum Result1<SuccessCode, FailureCode> {
        Success(SuccessCode),
        Failure(FailureCode),
        Uncertainty,
    }
    let mut _res = Result1::Success::<u32, u16>(12u32);

    println!("10.8 ===================");
    // Option<T>列挙体
    let mut v = vec![11, 22, 33];
    for _ in 0..5 {
        let item: Option<i32> = v.pop();
        match item {
            Some(number) => println!("{}, ", number),
            None => println!("#, "),
        }
    }

    println!("10.9 ===================");
    // Result<T, E>列挙体
    fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
        if denominator == 0. {
            Err(format!("Divide by zero"))
        } else {
            Ok(numerator / denominator)
        }
    }

    fn show_divide(num: f64, den: f64) {
        match divide(num, den) {
            Ok(val) => println!("{} / {} = {}", num, den, val),
            Err(msg) => println!("Cannnot divide {} by {}: {}", num, den, msg),
        }
    }

    println!("{:?}, {:?}", divide(8., 2.), divide(8., 0.));

    println!("10.10 ==================");
    // 列挙体の標準ユーティリティ関数
    fn divide2(numerator: f64, denominator: f64) -> Result<f64, String> {
        if denominator == 0. {
            Err(format!("Divide by zero"))
        } else {
            Ok(numerator / denominator)
        }
    }
    let r1 = divide2(8., 2.);
    let r2 = divide2(8., 0.);
    println!("{} {}", r1.is_ok(), r1.is_err());
    println!("{} {}", r2.is_ok(), r2.is_err());
    println!("{}", r1.unwrap());
    // println!("{}", r2.unwrap());  // パニック発生

    let mut a = Some(12);
    println!("{} {};", a.is_some(), a.is_none());
    a = None;
    println!("{} {}", a.is_some(), a.is_none());
}
