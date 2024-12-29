#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Resize(u32),
    Move(i32),
    Echo((i32,i32)),
    ChangeColor(i32),
    Quit,
}

fn main() {
    println!("{:?}", Message::Resize(19));
    println!("{:?}", Message::Move(12));
    println!("{:?}", Message::Echo((127,12)));
    println!("{:?}", Message::ChangeColor(12));
    println!("{:?}", Message::Quit);
}
