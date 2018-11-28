///
/// Lession 4 - Ownership & Borrowing
/// 

#[path = "./common.rs"]
mod common;

pub fn exec(){
    common::display_lesson_name(4);
    lesson();
}

fn lesson(){
    
    // a owns 2 and 2 is stored on the stack
    let a = 2; 
    let a = a + 3; // Shadowing
    let b = a; // inexpensive to copy so stores a copy

    println!("a = {} & b = {}", a, b);

    // example of data stored on heap
    let c = String::from("imaginea");
    // let d = c; // c no longer exists, it has been moved and will throw an error if we try to access it
    let d = &c; // get a reference to c, prevents a move

    println!("c = {} & d = {}", c, d);

    {
        let e = 10; // e is defined in this scope and will not be available outside it
        println!("e = {}", e);
    }

    // println!("e = {}", e); // will throw an error

    let mut dynamic_arr: Vec<i32> = Vec::new();

    for i in 1..10{ dynamic_arr.push(i) }
    
    simple_store(dynamic_arr[0]);
    ref_store(&dynamic_arr); // borrows rather than owning
    let dynamic_arr = store(dynamic_arr); // ownership returned
    
    println!("{:?}", dynamic_arr); // will throw an error as ownership has been moved to store

    println!("{:?}", &dynamic_arr[4..6]); // slices allow you to get a subset of data without owning it

}

fn store(v: Vec<i32>) -> Vec<i32>{
    println!("Stored {:?}", v);
    v
}

fn ref_store(v: &Vec<i32>){
    println!("Stored {:?}", v);
}

fn simple_store(i: i32){
    println!("Stored {}", i);
}

