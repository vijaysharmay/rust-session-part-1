#![allow(dead_code)]
///
/// Lession 7 - Modules
/// 

#[path = "./common.rs"]
mod common;

pub fn exec(){
    common::display_lesson_name(7);
    lesson();
}

fn lesson(){
    
    mod a{
        mod b{
            fn b1(){}
            fn b2(){}
            mod c{
                fn c1(){}
            }
        }
        fn a1(){}
        fn b2(){}
    }

    // println!("{:?}", a::b::b1()); // will throw an error as all are private functions

    mod x{
        pub mod y{
            pub fn y1(){}
            fn y2(){}
            mod z{
                fn z1(){}
            }
        }
        fn x1(){}
        fn x2(){}
    }

    println!("{:?}", x::y::y1()); // will not throw an error

    // use x::y::y1;

    // y1(); // y1 can be called directly

}

