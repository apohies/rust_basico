use std::num::ParseIntError;

fn main (){

    let swamini : &str = "4";

    try_parse(swamini);


    print!("the value of variable is {}",try_parse(swamini)  );


}

pub fn try_parse(evaluate: &str )-> u32{

    let mut guess: Result<u32, ParseIntError> = evaluate.parse::<u32>();

    if guess.is_err() {
        guess = Ok(0);
    }

    return guess.unwrap_or(0)

}