fn main() {
    println!("6.4 ====================");
    let a: i8 = 5;
    let b: i8 = 5;
    println!("{}", a + b);

    println!("6.5 ====================");
    // 符号なし整数
    let c: u8 = 5;
    let d: u8 = 5;
    println!("{}", c + d);

    println!("6.6 ====================");
    let arr = [11, 22, 33];
    let i: usize = 2;
    println!("{}", arr[i]);

    println!("6.7 ====================");
    let arr2 = [0];
    let i2 = 0;
    println!("{}", arr2[i2]);

    println!("6.9 ====================");
    let a = 4.6;
    let mut _b: f32 = 3.91e5;
    _b = a;
    println!("{}", _b);

    println!("6.10 ===================");
    // 明示的な型変換
    let a: i16 = 12;
    let b: u32 = 4;
    let c: f32 = 3.7;
    print!("{}", a as i8 + b as i8 + c as i8);
}
