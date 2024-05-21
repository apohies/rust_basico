mod structures{
    pub mod person;

}
use structures::person::Person;
fn main() {

    let mut persons : Vec<Person> = Vec::new();

    for i  in 0..3{

        let mut person1 = Person{
            name : String::from("pepe mel") + &i.to_string() , lastname : String::from("cheapen") + &i.to_string()
        };

        persons.push(person1);

    }

    for person in &persons {
        println!("firstName: {}, lastName: {}", person.name, person.lastname);
    }

}