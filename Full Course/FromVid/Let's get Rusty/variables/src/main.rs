fn main() {
    let tup: (&str, i32) = ("BRUH", 1000);
    let (some_words,subcount) = tup;
    
    let error_codes: [i32; 3]= [200,404,505];
    let success_code = error_codes[0];
    
    println!("{}, {}",some_words, subcount);
    println!("{}",success_code);
        
    let condition = true;
    let number = if condition {5} else {6};
    println!("{}",a_simple_function(11, 40));
    println!("{}",number);

    let mut counter = 0;
    let results = loop{
        counter +=1;
        if counter == 10 {
            break counter;
        }
    };
    
    println!("{}", results);
    
    let mut number = 3;
    while number != 0{
        println!("{}",number);
        number -=1;
    };
    
    println!("{:-<5}",'a');
    let a = [1,2,4,10,5,2];
    for x in a.iter(){
        println!("{}",x)
    }
    
    println!("{:-<5}",'a');
    for number in 1..5{
        println!("{number}");
    }
    
}

fn a_simple_function(x: i32, y: i32)-> i32{
    println!("Another function!");
    x + y
}