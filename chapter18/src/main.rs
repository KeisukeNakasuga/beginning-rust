struct Person {
    personal_names: String,
    family_names: String,
}

// implブロックの中にメソッドを定義して、
// データと振る舞いを紐付ける
impl Person {
    // 関連関数
    // 静的メソッドに対応する
    // Rustでは引数を持たず、Self型のインスタンスを返すメソッドはnewと名づける
    fn new() -> Self {
        Self {
            personal_names: String::new(),
            family_names: String::new(),
        }
    }

    // C++のconstメソッドに相当する
    fn naming(&self) -> String {
        format!("{} {}", self.personal_names, self.family_names)
    }
}

// 関数宣言を別ブロックでもできる
impl Person {
    fn set_personal_names(&mut self, new_name: String) {
        self.personal_names = new_name;
    }
}

mod routines {
    pub fn f() -> u32 {
        g()
    }
    fn g() -> u32 {
        123
    }
}

fn f() {
    print!("f ");
    g();
    m::f();
    m::m::f();
}

fn g() {
    print!("g ");
}

mod m {
    pub fn f() {
        print!("1.f ");
        g();
        m::f();
        super::g();
    }

    fn g() {
        print!("1.g ");
    }

    pub mod m {
        pub fn f() {
            print!("2.f ");
            g();
            super::super::g();
            crate::g();
        }
        fn g() {
            print!("2.g ");
        }
    }
}

fn main() {
    println!("18.2 ===================");
    // メソッド宣言

    let person = Person {
        personal_names: "John".to_string(),
        family_names: "Doe".to_string(),
    };

    println!("{}", person.naming());

    println!("18.3 ===================");
    let mut person2 = Person::new();
    println!("[{}]", person2.naming());
    person2.personal_names = "John".to_string();
    person2.family_names = "Doe".to_string();
    println!("[{}]", person2.naming());
    person2.set_personal_names("Jane".to_string());
    println!("[{}]", person2.naming());

    println!("18.4 ===================");
    // mod, pub
    println!("{}", routines::f());
    f();

    println!("18.5 ===================");

    type Number = f32;
    fn f1(x: Number) -> Number {
        x
    }
    fn f2(x: Number) -> Number {
        x
    }
    let a: Number = 2.3;
    let b: Number = 3.4;
    println!("{} {}", f1(a), f2(b));
}
