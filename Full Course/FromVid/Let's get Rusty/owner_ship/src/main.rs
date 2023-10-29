fn main() {
    let s1 = String::from("BRUH");
    let len = calculate(&s1);
    println!("length : {}", s1);
}

fn calculate(s: &String) -> usize {
    let len = s.len();
    return len
}
