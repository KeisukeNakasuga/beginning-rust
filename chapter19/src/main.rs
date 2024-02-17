// 「HasSquareRootトレイトを実装していれば、sq_rootを呼び出せる」と言う意味
trait HasSquareRoot {
    fn sq_root(self) -> Self;
}

// 「f32型にHasSquareRootを実装する」と言う意味
impl HasSquareRoot for f32 {
    fn sq_root(self) -> Self {
        self.sqrt()
    }
}

impl HasSquareRoot for f64 {
    fn sq_root(self) -> Self {
        self.sqrt()
    }
}

// ジェネリックメソッドを宣言
// where はトレイト制約
// → 「ジェネリック型Numberは、HasSquareRootを実装しなくてはならない」
// → 引数Numberは必ずsq_rootを持っている
fn quartic_root<Number>(x: Number) -> Number
where
    Number: HasSquareRoot,
{
    x.sq_root().sq_root()
}

fn main() {
    println!("{} {}", quartic_root(100f64), quartic_root(100f32));
}
