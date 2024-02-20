struct CommunicationChannel {
    address: String,
    port: u16,
}

impl Drop for CommunicationChannel {
    fn drop(&mut self) {
        // デストラクタ
        println!("Closing port {}:{}", self.address, self.port);
    }
}

impl CommunicationChannel {
    // コンストラクタ
    fn create(address: &str, port: u16) -> CommunicationChannel {
        println!("Opening port {}:{}", address, port);
        CommunicationChannel {
            address: address.to_string(),
            port: port,
        }
    }
    fn send(&self, msg: &str) {
        println!(
            "Sent to {}:{} the message '{}'",
            self.address, self.port, msg
        );
    }
}

// 演算子の多重定義
mod complex {
    pub struct Complex<Num> {
        re: Num,
        im: Num,
    }
    impl<Num> Complex<Num> {
        pub fn from_re_im(re: Num, im: Num) -> Self {
            Self { re, im }
        }
        pub fn re(&self) -> &Num {
            &self.re
        }
        pub fn im(&self) -> &Num {
            &self.im
        }
    }
    impl<Num> std::ops::Add for Complex<Num>
    where
        Num: std::ops::Add<Output = Num>,
    {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Self {
                re: self.re + rhs.re,
                im: self.im + rhs.im,
            }
        }
    }
}

// コンポジションを使う
struct Text {
    characters: String,
}
impl Text {
    fn from(text: &str) -> Text {
        Text {
            characters: text.to_string(),
        }
    }
    fn draw(&self) {
        print!("{}", self.characters);
    }
}

struct BoxedText {
    text: Text,
    first: char,
    last: char,
}

impl BoxedText {
    fn with_text_and_borders(text: &str, first: char, last: char) -> BoxedText {
        BoxedText {
            text: Text::from(text), // データ構造の再利用 と コンストラクタの再利用(from)
            first: first,
            last: last,
        }
    }
}

trait Draw {
    fn draw(&self);
}

impl Draw for Text {
    fn draw(&self) {
        print!("{}", self.characters);
    }
}

impl Draw for BoxedText {
    fn draw(&self) {
        print!("{}", self.first);
        self.text.draw(); // メソッドの再利用(draw)
        print!("{}", self.last);
    }
}

// 静的ディスパッチ
// コンパイル時に決定
fn draw_text<T>(txt: T)
where
    T: Draw,
{
    txt.draw();
}

// 参照でもOK
fn draw_text2<T>(txt: &T)
where
    T: Draw,
{
    txt.draw();
}

// 動的ディスパッチ
// 実行時に決定
fn draw_text3(txt: &dyn Draw) {
    // dyn 動的ディスパッチ参照
    // 参照しているオブジェクトの型に基づいて、呼び出すメソッドを選択できるポインタ
    txt.draw();
}

fn main() {
    // コンストラクタとデストラクタ
    // let channel_a = CommunicationChannel::create("usb4", 879);
    // channel_a.send("Message 1");
    // {
    //     let channel_b = CommunicationChannel::create("eth1", 12000);
    //     channel_b.send("Message 2"); // ブロックを抜けるとオブジェクトが破棄される
    // }
    // channel_a.send("Message 3");

    // コンポジションを使う
    let greeting = Text::from("Hello");
    greeting.draw();

    let boxed_greeting = BoxedText::with_text_and_borders("Hi", '[', ']');
    print!(", ");
    boxed_greeting.draw();

    println!();

    // 静的ディスパッチ
    let greeting2 = Text::from("Hello");
    let boxed_greeting2 = BoxedText::with_text_and_borders("Hi", '[', ']');
    draw_text(greeting2);
    print!(", ");
    draw_text(boxed_greeting2);

    println!();

    // 参照でもOK
    let greeting3 = Text::from("Hello");
    let boxed_greeting3 = BoxedText::with_text_and_borders("Hi", '[', ']');
    draw_text2(&greeting3);
    print!(", ");
    draw_text2(&boxed_greeting3);

    println!();

    // 動的ディスパッチ
    let greeting4 = Text::from("Hello");
    let boxed_greeting4 = BoxedText::with_text_and_borders("Hi", '[', ']');
    draw_text3(&greeting4);
    print!(", ");
    draw_text3(&boxed_greeting4);
}
