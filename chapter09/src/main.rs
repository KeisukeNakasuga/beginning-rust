fn main() {
    println!("9.5 ====================");
    fn print_double(mut x: f64) {
        x *= 2.;
        println!("{}", x);
    }
    let x = 4.;
    print_double(x);
    println!("{}", x);

    println!("9.6 ====================");
    // 関数が返却する値の型を明記
    fn double(x: f64) -> f64 {
        x * 2.
    }
    println!("{}", double(17.3));

    println!("9.7 ====================");
    // 早期終了
    // Rustではreturnを最後の文に使わないのが良いフォーマット
    fn f(x: f64) -> f64 {
        if x <= 0. { 0. }
        else { x + 3. }
    }
    println!("{} {}", f(1.), f(-1.));

    println!("9.8 ====================");
    // 複数の値を返す
    fn divide(dividend: i32, divisor: i32) -> (i32, i32) {
        (dividend / divisor, dividend % divisor)
    }
    println!("{:?}", divide(50, 11));

    println!("9.9 ====================");
    fn double_arr(mut a: [i32; 10]) -> [i32; 10] {
        for i in 0..10 {
            a[i] *= 2;
        }
        a
    }
    let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
    arr = double_arr(arr);
    println!("{:?}", arr);

    println!("9.10 ===================");
    // 引数の参照渡し
    fn double_arr_ref(a: &mut [i32; 10]) {  // & オブジェクトのアドレス またmutを付与するのは後の処理で書き換えるから
        for n in 0..10 {
            a[n] *= 2;  // * はメモリアドレスaに存在するオブジェクト
        }
    }
    let mut arr_ref = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
    double_arr_ref(&mut arr_ref);
    println!("{:?}", arr_ref);

}
