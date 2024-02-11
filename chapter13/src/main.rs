use std::cmp::Ordering;

fn main() {
    println!("13.1 ===================");
    let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
    fn desc(a: &i32, b: &i32) -> Ordering {
        if a < b {
            Ordering::Greater
        } else if a > b {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
    arr.sort_by(desc);
    println!("{:?}", arr);

    println!("13.2 ===================");
    // 環境をキャプチャする
    // 次のコードはコンパイルできない
    // twoはletで宣言されており、変更される可能性があるため関数内から参照できない
    // let two = 2.;
    // fn print_double(x: f64) {
    //     println!("{}", x * two);
    // }
    // print_double(17.2);

    // constで宣言すれば有効
    const TWOA: f64 = 2.;
    fn print_double(x: f64) {
        println!("{}", x * TWOA);
    }
    print_double(17.2);

    // staticで宣言しても有効
    static TWOB: f64 = 2.;
    fn print_double_b(x: f64) {
        println!("{}", x * TWOB);
    }
    print_double_b(17.2);

    println!("13.3 ===================");
    // クロージャ
    let mut arr2 = [4, 8, 1, 10, 0, 45, 12, 7];
    let desc2 = |a: &i32, b: &i32| -> Ordering {
        if a < b {
            Ordering::Greater
        } else if a > b {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    };
    arr2.sort_by(desc2);
    println!("{:?}", arr2);

    // クロージャを使って書き直す
    let mut arr3 = [4, 8, 1, 10, 0, 45, 12, 7];
    arr3.sort_by(|a, b| {
        if a < b {
            Ordering::Greater
        } else if a > b {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });
    println!("{:?}", arr3);

    // 標準ライブラリを使って書き直す
    let mut arr4 = [4, 8, 1, 10, 0, 45, 12, 7];
    arr4.sort_by(|a, b| b.cmp(a));
    println!("{:?}", arr4);

    println!("13.4 ===================");
    // クロージャを呼び出す構文
    let factor = 2;
    let multiply = |a| a * factor;
    println!("{}", multiply(13));
    let multiply_ref = &multiply;
    println!(
        "{} {} {} {} {}",
        (*multiply_ref)(13),
        multiply_ref(13),
        (|a| a * factor)(13),
        (|a: i32| a * factor)(13),
        |a| -> i32 { a * factor }(13)
    );
}
