///
/// Lession 1 - Setup & Primitives
/// 

pub fn exec(){
    println!("");
    println!("{}", format!("{:*^1$}", "", 50));
    println!("{}", format!("{:*^1$}", " Executing Lesson 1 ", 50));
    println!("{}", format!("{:*^1$}", "", 50));
    println!("");
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

    // Boolean
    let c1: bool = true;
    let c2: bool = false;
    println!("c1 is {} and c2 is {}", c1, c2);

    // Arrays
    let arr = [21, 32, 3, 74];
    println!("First Element of array is {}", arr[0]);
    println!("Length of array is {}", arr.len());
    println!("Last Element of array is {}", arr[arr.len()-1]);

}

