
#[derive(Debug)]

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) -> &Message {
        self
    }
}


fn main() {
    let m = Message::Write(String::from("hello"));
    let call_value = m.call();

    println!("value of m.call(): {:?}", call_value);
}
