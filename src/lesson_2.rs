///
/// Lession 2 - Strings, Tuples & Arrays
/// 

#[path = "./common.rs"]
mod common;

pub fn exec(){
    common::display_lesson_name(2);
    lesson();
}

fn lesson(){
    
    // Strings & String Literals
    let a3 = "imaginea";
    let mut a4 = String::from(a3);
    a4.push_str(" technologies");
    println!("a3 -> {} & a4 -> {}", a3, a4);

    // Tuples
    let a5: (i32, u64) = (1, 2);
    let a6: (f32, i32, (i32, i32)) = (1.0, 2, (4, 5));
    let (_, _, (_, a7)) = a6;
    println!("{first:?} is a tuple and so is {second:?}", first=a6, second=a5);
    println!("Unpacked a7 = {}", a7);

    // Arrays
    let arr = [21, 32, 3, 74, 64, 9];
    println!("First Element of array is {}", arr[0]);
    println!("Length of array is {}", arr.len());
    println!("Last Element of array is {}", arr[arr.len()-1]);
    println!("The 2nd, 3rd & 4th Elements are {:?}", &arr[1..4]);

}

