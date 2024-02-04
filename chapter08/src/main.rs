fn main() {
    println!("8.1 ====================");
    // 8.1 タプル
    // タプル内要素の型は違ってもOK
    let data: (i32, f64, char) = (10000, 183.19, 'Q');
    let copy_of_data: (i32, f64, char) = data;
    println!("{}, {}, {}", data.0, copy_of_data.1, copy_of_data.2);

    // ミュータブル
    let mut data2 = (10000, 183.19, 'Q');
    data2.0 = -5;
    println!("{}, {}, {}", data2.0, data2.1, data2.2);

    println!("8.2 ====================");
    // 構造体
    struct SomeData {
        integer: i32,
        fractional: f32,
        character: char,
        five_bytes: [u8; 5],
    }
    let data3 = SomeData {
        integer: 10_000_000,
        fractional: 183.19,
        character: 'Q',
        five_bytes: [9, 0, 250, 60, 200],
    };
    println!("{}, {}, {}, {}", data3.five_bytes[3], data3.integer, data3.fractional, data3.character);

    println!("8.3 ====================");
    // タプル構造体
    struct SomeData2 (
        i32,
        f32,
        char,
        [u8; 5],
    );
    let data4 = SomeData2(
        10_000_000,
        183.19,
        'Q',
        [9, 0, 250, 60, 200],
    );
    println!("{}, {}, {}, {}", data4.2, data4.0, data4.1, data4.3[2]);
}
