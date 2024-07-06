
use crate::structures::rectangle::Rectangle;

mod structures{
    pub mod rectangle;

}





fn main(){

    let rectangle = Rectangle {
         width : 20,
        height: 2,
    };



    print!("the area of rectangle is : {}", rectangle.area());

}

fn area_of_rectangle( rectangle : &Rectangle ) -> i32 {

    let area : i32 = rectangle.width * rectangle.height;

    return area;
}