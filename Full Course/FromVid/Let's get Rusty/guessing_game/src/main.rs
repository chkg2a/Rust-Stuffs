use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    // let secret_num = rand::thread_rng().gen_range(1..101);
    let secret_num = something();
    println!("Computer Num: {}", secret_num);
    loop{
        let mut guess = String::new();
        println!("Pleas input your guess!");
        io::stdin()
            .read_line(&mut guess)
            .expect("Assignment to guess failed");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("My guess: {}",guess);
        match guess.cmp(&secret_num){
            Ordering::Less => println!("Your guess is too small"),
            Ordering::Greater => println!("Your guess is too big"), 
            Ordering::Equal => {
                println!("You guessed it right");
                break;
            },
        }
    }
}

fn something() -> u32{
   
    let secret_num = rand::thread_rng().gen_range(1..101);
    println!("{}",secret_num);
    
    return secret_num
}
