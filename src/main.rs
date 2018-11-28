use std::env::args;
use std::collections::HashMap;

mod lesson_1;
mod lesson_2;
mod lesson_3;
mod lesson_4;
mod lesson_5;
mod lesson_6;
mod lesson_7;
mod lesson_8;
mod lesson_9;
mod lesson_10;

fn main(){
    
    let mut method_mapping: HashMap<i32, fn() -> ()> = HashMap::new();

    method_mapping.insert(1, lesson_1::exec);
    method_mapping.insert(2, lesson_2::exec);
    method_mapping.insert(3, lesson_3::exec);
    method_mapping.insert(4, lesson_4::exec);
    method_mapping.insert(5, lesson_5::exec);
    method_mapping.insert(6, lesson_6::exec);
    method_mapping.insert(7, lesson_7::exec);
    method_mapping.insert(8, lesson_8::exec);
    method_mapping.insert(9, lesson_9::exec);
    method_mapping.insert(10, lesson_10::exec);

    let args: Vec<String> = args().collect();
    let lesson_number = &args[1].trim().parse();
    let lesson_number = match lesson_number{
        Ok(num) => num,
        Err(_) => panic!("something went wrong"),
    };

    match method_mapping.get(lesson_number){
        Some(lesson) => lesson(),
        None => panic!("Lesson number not found"),
    }
    
}