enum IpAddrKind{
    V4(String),
    V6(String)
}

enum Message{
    Quit,
    Move {x: u32, y: u32},
    Write(String),
    ChangeColor(i32,i32,i32)
}

impl Message{
    fn some_function(){
        println!("bruh");
    }
}

struct IpAddr{
    kind: IpAddrKind,
    ip: String
}

fn main() {
    let my_ip = IpAddrKind::V4(String::from("127.0.0.1"));
    Message.some_function()
}
