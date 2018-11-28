///
/// Lession 5 - Structs, Traits & Generics
/// 

#[path = "./common.rs"]
mod common;

pub fn exec(){
    common::display_lesson_name(5);
    lesson();
}

fn lesson(){
    
    trait Shape{
        fn area(&self) -> f32;
        fn display(&self){
            println!("Area = {:.2}", self.area());
        }
    }

    struct Rectangle{
        length: i32,
        width: i32
    }

    struct Square{
        length: i32
    }

    struct Circle{
        radius: i32
    }

    impl Shape for Rectangle{
        fn area(&self) -> f32{
            (self.length * self.width) as f32
        }
    }

    impl Shape for Square{
        fn area(&self) -> f32{
            (self.length.pow(2)) as f32
        }
    }
    
    impl Shape for Circle{
        fn area(&self) -> f32{
            3.14 * (self.radius.pow(2) as f32)
        }
    }

    let a = Rectangle{length: 20, width: 30};
    let b = Square{length: 20};
    let c = Circle{radius: 20};

    a.display();
    b.display();
    c.display();
}

