pub struct Rectangle {
    pub width: i32,
    pub height: i32,
}


impl Rectangle {
    pub fn area(&self) -> i32 {
        self.width * self.height
    }
}