// This is a simple macro named `say_hello`.
macro_rules! say_hello {
    () => {
        println!("HEllo WOrld!");
    }
}

fn main(){
    let pi = 3.141234123;
    println!("{:.3}",pi);
    say_hello!();
}
