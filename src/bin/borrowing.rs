use std::ffi::CString;
use crate::structures::person::Person;

mod structures{
    pub mod person;

}
fn main (){


    let mut x = Person { name : String::from("saido") , lastname : String::from("meme") } ;
    cambiador_nivel( &mut x.name);
}

fn cambiador_nivel (s : &mut String) -> String
{

    format!("Nuevo {}", s)

}