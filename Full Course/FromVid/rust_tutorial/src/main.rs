#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write,BufRead,BufReader,ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;

enum Days{
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday
}    

impl Days{
   fn is_weekend(&self) -> bool {
        match self{
            Days::Saturday | Days::Sunday => true,
            _ => false
        }
    }
}


fn main(){
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1,2,3,4,5];
    let second: &i32 = &vec2[2];
    vec2.push(5);

    match vec2.get(0){
        Some(second) => println!("2nd: {}",second),
        None => println!("bruh")
    }
}
