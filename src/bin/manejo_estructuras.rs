use crate::structures::animal::Animal;

mod structures{
    pub mod person;
    pub mod animal;

}
fn main() {

    let a1 = Animal { name: String::from("cat"), age :2 };

    let a2 =  Animal { name : a1.name.clone() + "man" , age : a1.age};

    print!("el animal es :{} \n", a1.name );

    print!("el animal es  {}",a2.name);
}

