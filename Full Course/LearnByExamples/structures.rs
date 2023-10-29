#[derive(Debug)]
struct BAChar {
    name: String,
    age: u8,
}

fn main(){
    let name1 = String::from("Izuna");
    let age1 = 24;

    let char1 = BAChar { name=name1, age:age1};

    println!("{:?}",char1);

}
