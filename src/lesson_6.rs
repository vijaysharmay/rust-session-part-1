///
/// Lession 6 - Enums, Option & Result
/// 

#[path = "./common.rs"]
mod common;

pub fn exec(){
    common::display_lesson_name(6);
    lesson();
}

fn lesson(){
    
    enum Direction{
        Up(char),
        Down(char),
        Left(char),
        Right(char),
    }

    fn set_direction(key: char) -> Direction{
        match key{
            'w' => Direction::Up('w'),
            's' => Direction::Down('s'),
            'a' => Direction::Left('a'),
            'd' => Direction::Right('d'),
            _   => panic!("Only w,a,s & d are allowed")
        }
    }

    let key = common::get_char_value_from_user(String::from("Press either w,a,s or d"));
    let direction = set_direction(key);

    match direction{
        Direction::Up(_) => println!("Pressed w"),
        Direction::Down(_) => println!("Pressed s"),
        Direction::Left(_) => println!("Pressed a"),
        Direction::Right(_) => println!("Pressed d"),
    };

    let x = Some(2);

    let y = match x{
        Some(n) => n,
        None => panic!("something went wrong")
    };

    println!("y = {}", y);

    fn divide_by_number(numerator:i32, denominator: i32) -> Option<f32>{
        if denominator == 0{
            None
        }else{
            Some(numerator as f32 / denominator as f32)
        }
    }

    let num = common::get_numeric_value_from_user(String::from("Enter a number"));
    let den = common::get_numeric_value_from_user(String::from("Enter another number"));

    match divide_by_number(num, den){
        Some(n) => println!("2/3 is {}", n),
        None => panic!("Tried dividing by zero")
    }

    #[derive(Debug)]
    enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    type MathResult = Result<f64, MathError>;

    fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }

    fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    fn ln(x: f64) -> MathResult {
        if x <= 0.0 {
            Err(MathError::NonPositiveLogarithm)
        } else {
            Ok(x.ln())
        }
    }
    
    fn op(x: f64, y: f64) -> f64 {
        match div(x, y) {
            Err(why) => panic!("{:?}", why),
            Ok(ratio) => match ln(ratio) {
                Err(why) => panic!("{:?}", why),
                Ok(ln) => match sqrt(ln) {
                    Err(why) => panic!("{:?}", why),
                    Ok(sqrt) => sqrt,
                },
            },
        }
    }

    println!("{}", op(30.0, 10.0));

}



