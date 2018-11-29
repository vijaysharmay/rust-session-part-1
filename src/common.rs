#![allow(dead_code)]

use std::io;

pub fn display_lesson_name(lesson_number: i32){
    let title = format!(" Executing Lesson {} ", lesson_number);
    println!("");
    println!("{}", format!("{:*^1$}", "", 50));
    println!("{}", format!("{:*^1$}", title, 50));
    println!("{}", format!("{:*^1$}", "", 50));
    println!("");
}

pub fn get_numeric_value_from_user(message: String) -> i32{

    println!("{}", format!("{}", message));
    
    let mut val = String::new();

    io::stdin().read_line(&mut val).expect("Failed to read line");

    let parsed_val = match val.trim().parse::<i32>(){
        Ok(num) => num,
        Err(_) => panic!("Something went wrong"),
    };

    parsed_val as i32
}

pub fn get_char_value_from_user(message: String) -> char{

    println!("{}", format!("{}", message));
    
    let mut val = String::new();

    io::stdin().read_line(&mut val).expect("Failed to read line");

    let parsed_val = match val.trim().parse::<char>(){
        Ok(num) => num,
        Err(_) => panic!("Something went wrong"),
    };

    parsed_val as char
}