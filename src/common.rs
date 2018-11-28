pub fn display_lesson_name(lesson_number: i32){
    let title = format!(" Executing Lesson {} ", lesson_number);
    println!("");
    println!("{}", format!("{:*^1$}", "", 50));
    println!("{}", format!("{:*^1$}", title, 50));
    println!("{}", format!("{:*^1$}", "", 50));
    println!("");
}