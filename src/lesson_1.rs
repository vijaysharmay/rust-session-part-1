///
/// Lession 1 - Setup & Primitives
/// 

#[path = "./common.rs"]
mod common;

pub fn exec(){
    common::display_lesson_name(1);
    lesson();
}

fn lesson(){
    
    // Integers (i8 to i64, u8 to u64, isize, usize)
    let a1: i32 = 1;
    let a2: usize = 2;
    println!("a1 = {} & a2 = {}", a1, a2);

    // Floating point numbers
    let b1: f32 = 1.0;
    let b2: f64 = 2.0;
    println!("b1 = {:.3} & b2 = {:.2}", b1, b2);

    // Characters
    let d1: char = 'c';
    println!("Single character - {}", d1);

    // Boolean
    let c1: bool = true;
    let c2: bool = false;
    println!("c1 is {} and c2 is {}", c1, c2);

    // Constants
    let tech: String = String::from("Rust");
    println!("Current technology used is {}", tech);

}

