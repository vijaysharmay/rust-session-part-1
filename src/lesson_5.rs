///
/// Lession 5 - Structs, Traits & Generics
/// 

#[path = "./common.rs"]
mod common;

use std::ops::Mul;
use std::fmt;

pub fn exec(){
    common::display_lesson_name(5);
    lesson();
}

fn lesson(){
    #[allow(dead_code)]
    struct CommonShape; // unit-like struct
    
    // Interfaces
    trait Shape<T> where T: fmt::Display{
        fn area(&self) -> T;
        fn display(&self){
            println!("Area = {:.2}", self.area());
        }
    }

    #[derive(Debug)] // derives the Debug Trait
    struct Rectangle<T: Mul>{
        length: T,
        width: T
    };

    struct Square<T: Mul>(T); // Tuple Structs

    struct Circle<T: Mul>{
        radius: T
    };

    // Creates a name space
    impl<T> Rectangle<T> where T: Mul<Output = T>{
        fn new(length: T, width: T) -> Rectangle<T>{
            Rectangle{
                length,
                width
            }
        }
        fn custom(&self){
            println!("This is a custom function for Rectangles");
        }
    };

    impl<T> Square<T> where T: Mul<Output = T>{
        fn custom(&self){
            println!("This is a custom function for Squares");
        }
    };

    impl<T> Circle<T> where T: Mul<Output = T>{
        fn custom(&self){
            println!("This is a custom function for Circle");
        }
    };

    impl<T: Mul> Shape<T> for Rectangle<T> where T: Mul<Output = T> + fmt::Display + Copy{
        fn area(&self) -> T{
            self.length * self.width
        }
    };

    impl<T: Mul<Output = T> + fmt::Display + Copy> Shape<T> for Square<T>{
        fn area(&self) -> T{
            self.0 * self.0
        }
    };
    
    impl<T: Mul<Output = T> + fmt::Display + Copy> Shape<T> for Circle<T>{
        fn area(&self) -> T{
            self.radius * self.radius
        }
    };

    let a = Rectangle::new(20, 30);
    let b = Square(20);
    let c = Circle{radius: 20};

    println!("{:?}", a);
    println!("{:#?}", a);

    a.display();
    b.display();
    c.display();

    a.custom();
    b.custom();
    c.custom();

}

