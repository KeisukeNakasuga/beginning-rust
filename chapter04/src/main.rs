fn main() {
    // 4.1 if
    let n = 4;
    if n > 0 {
        print!("positive!");
    } else {
        print!("not positive");
    }

    // 4.2 条件式
    let n = 5;
    print!("{}",
        if n > 1000 {
            "big"
        }
        else if n > 0 {
            "small"
        }
        else if n < 0 {
            "negative"
        }
        else {
            "neither positive nor negative"
        }
    );

    // 4.3
    let mut n = 1;
    while n <= 10 {
        print!("{} ", n * n);
        n += 1;
    }

    let mut nn = 0;
    while nn < 50 {
        nn += 1;
        if nn % 3 == 0 { continue; }
        if nn * nn > 400 { break; }
        print!("{} ", nn * nn);
    }

    // 4.5 カウント付きループ
    for n in 1..11 {
        print!("{} ", n * n);
    }

    println!("========================");

    for n in 1..=10 {
        print!("{} ", n * n);
    }

    println!("========================");

    let mut limit = 4;
    for n in 1..limit + 2 {
        limit -= 1;
        print!("{} {}, ", limit, n);
    }
    print!("{}", limit);

    println!("========================");
}
