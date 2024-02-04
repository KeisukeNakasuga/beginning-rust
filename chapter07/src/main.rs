fn main() {
    println!("7.1 ====================");
    // 列挙
    #[allow(dead_code)]
    enum Contient {
        Europe,
        Asia,
        Africa,
        America,
        Oceania,
    }

    let mut contin = Contient::Asia;
    match contin {
        Contient::Europe => {
            contin = Contient::Asia;
            println!("E");
        },
        Contient::Asia => {
            let a = 7;
            println!("{}", a);
        },
        Contient::Africa => println!("Af"),
        Contient::America => println!("Am"),
        Contient::Oceania => println!("O")
    }

    println!("7.6 ====================");
    enum Result {
        Success(u8),
        Failure(u16, char),
        Uncertainty,
    }

    let outcome = Result::Failure(20, 'X');
    match outcome {
        Result::Success(0) => println!("Result: 0"),
        Result::Success(1) => println!("Result: 1"),
        Result::Success(_) => println!("Result: other"),
        Result::Failure(10, 'X') => println!("Error: 10 X"),
        Result::Failure(10, _) => println!("Error: 10"),
        Result::Failure(_, 'X') => println!("Error: X"),
        Result::Failure(_, _) => println!("Error: other"),
        Result::Uncertainty => {},
    }

    println!("7.7 ====================");
    // パターン内に変数がある
    #[allow(dead_code)]
    enum Result2 {
        Success(u8),
        Failure(u16, char),
        Uncertainty,
    }
    let outcome2 = Result::Success(13);
    match outcome2 {
        Result::Success(0) => println!("Result: 0"),
        Result::Success(1) => println!("Result: 1"),
        Result::Success(n) => println!("Result: {}", n),
        Result::Failure(10, 'X') => println!("Error: 10 X"),
        Result::Failure(10, m) => println!("Error: 10 in module {}", m),
        Result::Failure(code, 'X') => {
            println!("Error: n.{} X", code)
        },
        Result::Failure(code, module) => {
            println!("Errpr: n.{} in module {}", code, module);
        },
        Result::Uncertainty => {},
    }

    println!("7.8 ====================");
    // match式
    #[allow(dead_code)]
    enum CardinalPoint { North, South, West, East }
    let direction = CardinalPoint::South;
    println!("{}", match direction {
        CardinalPoint::North => 'N',
        CardinalPoint::South => 'S',
        _ => '*',
    });

    println!("7.9 ====================");
    // match構造でガード
    for n in -2..5 {
        println!("{} is {}.", n, match n {
            0 => "zero",
            1 => "one",
            _ if n < 0 => "negative",
            _ => "plural",
        });
    }

    println!("7.10 ===================");
    #[allow(dead_code)]
    enum E {
        Case1(u32),
        Case2(char),
        Case3(i64, bool),
    }
    let v = E::Case3(1234, true);
    match v {
        E::Case3(n, b) => if b {
            println!("{}", n);
        }
        _ => {}
    }

    if let E::Case3(n, b) = v {  // E::Case3とvを比較
        if b { println!("{}, {}", n, b) }  // matchしたものを取り出してブロックを実行
    }

    #[allow(dead_code)]
    enum E2 {
        Case1(u32),
        Case2(char),
    }
    let mut v = E::Case1(0);
    let mut v2 = E::Case2('c');
    while let E::Case1(n) = v {
        println!("{}", n);
        if n == 6 { break; }
        v = E::Case1(n+1);
    }
}
