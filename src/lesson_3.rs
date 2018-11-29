///
/// Lession 3 - Control Flow, Conditionals & Pattern Matching
/// 

#[path = "./common.rs"]
mod common;

use std::cmp::Ordering;

pub fn exec(){
    common::display_lesson_name(3);
    lesson();
}

fn lesson(){
    
    // get value from user
    let mut a = common::get_numeric_value_from_user(String::from("Please enter a value"));
    println!("Value of a is {}", a);

    // If condition
    if a > 10{
        println!("Value of a is greater than 10");
    }else{
        println!("Value of a is less than 10");
    }

    // Simple loop
    loop{
        if a == 0 {
            break
        }else{
            a -= 1
        }
    }
    println!("Value of a after simple loop is {}", a);

    // while loop
    while a == 0 {
        a += 1
    }
    println!("Value of a after while loop is {}", a);

    // for loop
    for i in 1..10{
        println!("{}", i);
    }

    // Expressions
    let b = if a == 1{
        2
    }else{
        0
    };
    println!("Value of b is {}", b);

    // match statement
    let c = common::get_numeric_value_from_user(String::from("Please enter another value"));

    match c.cmp(&a){
        Ordering::Greater => println!("Second value from user is greater than the first value"),
        Ordering::Less => println!("Second value from user is lesser than the first value"),
        Ordering::Equal => println!("What a guess macha"), 
    }
}

