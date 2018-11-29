///
/// Lession 10 - Collections
/// 

#[path = "./common.rs"]
mod common;

// Sequences
use std::collections::VecDeque;

// Maps
use std::collections::HashMap;

// Sets
use std::collections::HashSet;

// Misc
use std::collections::BinaryHeap;

pub fn exec(){
    common::display_lesson_name(9);
    lesson();
}

fn lesson(){
    
    // Vectors
    let mut v1: Vec<i32> = Vec::new();
    for i in 1..10{
        v1.push(i)
    }
    println!("Vector v1 = {:?}", v1);

    // Dequeues
    let mut v2: VecDeque<i32> = VecDeque::new();
    for i in 1..10{
        if i % 2 == 0 {
            v2.push_back(i)
        }else{
            v2.push_front(i)
        }
    }
    println!("Dequeue v2 = {:?}", v2);

    v2.pop_front();
    v2.pop_back();
    v2.pop_back();

    println!("Dequeue v2 after 1 pop front and 2 pop back = {:?}", v2);

    // Hashmaps & BTreemaps have almost the same syntax
    let mut h1: HashMap<i32, i32> = HashMap::new();

    for i in 1..10{
        h1.insert(i, i.pow(2));
    }

    let key = common::get_numeric_value_from_user(String::from("Enter a value between 1 and 10"));

    match h1.get(&key){ // get function returns an option
        Some(n) => println!("Found value {} for key {}", n, key),
        None => println!("No such key in the hashmap")
    }

    let key = common::get_numeric_value_from_user(String::from("Enter a key you want to remove"));

    h1.remove(&key);

    println!("Hashmap after removing key = {}", key);

    for (k, v) in &h1{
        println!("{}: {}", k, v)
    }

    // Hashsets are Hashmaps where value is ()

    let mut books = HashSet::new();

    // Add some books.
    books.insert("A Dance With Dragons".to_string());
    books.insert("To Kill a Mockingbird".to_string());
    books.insert("The Odyssey".to_string());
    books.insert("The Great Gatsby".to_string());

    // Check for a specific one.
    if !books.contains("The Winds of Winter") {
        println!("We have {} books, but The Winds of Winter ain't one.",
                books.len());
    }

    // Remove a book.
    books.remove("The Odyssey");

    // Iterate over everything.
    for book in &books {
        println!("{}", book);
    }

    // BinaryHeap is a max-heap implementation

    let mut bh: BinaryHeap<i32> = BinaryHeap::new();

    for i in 1..10000{
        bh.push(i)
    }

    match bh.peek(){
        Some(n) => {
            println!("Max Val = {}", n)
        },
        None => println!("Something went wrong")
    }

    

}

