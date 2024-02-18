// 「HasSquareRootトレイトを実装していれば、sq_rootを呼び出せる」と言う意味
trait HasSquareRoot {
    fn sqrt(self) -> Self;
}

// 「f32型にHasSquareRootを実装する」と言う意味
impl HasSquareRoot for f32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

impl HasSquareRoot for f64 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

trait HasAbsoluteValue {
    fn abs(self) -> Self;
}

impl HasAbsoluteValue for f32 {
    fn abs(self) -> Self {
        self.abs()
    }
}

impl HasAbsoluteValue for f64 {
    fn abs(self) -> Self {
        self.abs()
    }
}

// 複数のメソッドをもつパターン
// trait HasSqrtAndAbs {
//     fn sqrt(self) -> Self;
//     fn abs(self) -> Self;
// }
//
// impl HasSqrtAndAbs for f32 {
//     fn sqrt(self) -> Self {
//         self.sqrt()
//     }
//
//     fn abs(self) -> Self {
//         self.abs()
//     }
// }
//
// impl HasSqrtAndAbs for f64 {
//     fn sqrt(self) -> Self {
//         self.sqrt()
//     }
//
//     fn abs(self) -> Self {
//         self.abs()
//     }
// }

fn abs_quartic_root<Number>(x: Number) -> Number
where
    Number: HasSquareRoot + HasAbsoluteValue,
{
    x.abs().sqrt().sqrt()
}

// ジェネリックメソッドを宣言
// where はトレイト制約
// → 「ジェネリック型Numberは、HasSquareRootを実装しなくてはならない」
// → 引数Numberは必ずsq_rootを持っている
// fn quartic_root<Number>(x: Number) -> Number
// where
//     Number: HasSquareRoot,
// {
//     x.sq_root().sq_root()
// }

// トレイト継承
trait HasSqrtAndAbs: HasSquareRoot + HasAbsoluteValue {}

// 19.10 ジェネリックトレイト
// トレイトを宣言
trait HasLnExpMultiply {
    fn ln(self) -> Self;
    fn exp(self) -> Self;
    fn multiply(self, other: Self) -> Self;
}

impl HasLnExpMultiply for f64 {
    fn ln(self) -> Self {
        self.ln()
    }

    fn exp(self) -> Self {
        self.exp()
    }

    fn multiply(self, other: Self) -> Self {
        self * other
    }
}

impl HasLnExpMultiply for f32 {
    fn ln(self) -> Self {
        self.ln()
    }

    fn exp(self) -> Self {
        self.exp()
    }

    fn multiply(self, other: Self) -> Self {
        self * other
    }
}

fn exponentiate<Number>(base: Number, exponent: Number) -> Number
where
    Number: HasLnExpMultiply,
{
    (base.ln().multiply(exponent)).exp()
}

trait Dictionary<Key> {
    fn get(&self, key: Key) -> Option<String>;
}

struct Record {
    id: u32,
    name: String,
}

struct RecordSet {
    data: Vec<Record>,
}

fn get_name<D>(dict: &D, id: u32) -> Option<String>
where
    D: Dictionary<u32>,
{
    dict.get(id)
}

fn main() {
    // println!("{} {}", quartic_root(100f64), quartic_root(100f32));

    println!("19.5 ===================");
    println!(
        "{} {}",
        abs_quartic_root(-100f64),
        abs_quartic_root(-100f32)
    );

    println!("19.10 ===================");
    println!("{} {}", exponentiate(2.5, 3.2), exponentiate(2.5f32, 3.2));
}
