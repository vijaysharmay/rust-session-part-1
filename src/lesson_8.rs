///
/// Lession 8 - Closures & Iterators
/// 

#[path = "./common.rs"]
mod common;

pub fn exec(){
    common::display_lesson_name(8);
    lesson();
}

fn lesson(){
    
    // A simple anonymous function
    let add = |x, y| x + y;
    let display = || println!("Simple display closure");

    println!("Added 2 and 3 gives {}", add(2,3));

    let apply_func = |f: fn(i32, i32) -> i32, x: i32, y: i32| { 
        println!("Function {:?} has been executed on {} and {} and the result is {}", f, x, y, f(x,y)) 
    };

    apply_func(add, 2, 3);
    display();

    let x = |i| i * 10;
    fn apply<F>(f: F, x: i32) -> i32 where F: Fn(i32) -> i32 { f(x) }

    struct Point<F: Fn(i32, i32) -> f32>{
        x: i32,
        y: i32,
        area: F
    }

    let area = |x:i32, y:i32| -> f32 {
        (x * y) as f32
    };
    
    let point = Point{
        x: 2,
        y: 3,
        area
    };

    let area = (point.area)(2,3);

    println!("Point created with x = {}, y = {} and area = {}", point.x, point.y, area);

    let k = apply(x, 20);
    println!("k = {}", k);

    let mut counter = 0;

    let mut increment = || counter+= 1; // mutable borrow
    // let mut decrement = || counter-= 1; // will fail because of above mutable borrow

    increment();
    increment();
    increment();

    println!("Counter = {}", counter);

    // Iterators

    let v = vec![1,2,3];
    println!("does the vector {:?} have 2? {}", v, v.iter().any(|&x| x != 2)); // checks through the closure if there are any 2's in v

    // Iterator Trait implementation in Rust

    // trait Iterator{
    //     type Item;
    //     fn next(&mut self) -> Option<Self::Item>;
    // }

    struct Fib{
        c: i32,
        n: i32
    }

    impl Iterator for Fib{
        type Item = i32;

        fn next(&mut self) -> Option<i32>{
            let tmp = self.c;
            self.c = self.c + self.n;
            self.n = tmp;
            Some(self.c)
        }
    }

    let mut f = Fib{c: 0, n: 1};

    for i in 1..10{
        println!("{}th fibonacii number is {}", i, f.next().unwrap());
    }    
    
    // sum of all squares of even numbers below 1000

    let sum = (0..)                
                .take_while(|&n| n < 1000)
                .filter(|&n| n % 2 == 0)
                .map(|n| n*n)
                .fold(0, |c, a| c+ a);

    println!("Sum of of all squares of even numbers below 1000 is {}", sum);


}

