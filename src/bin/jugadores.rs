use crate::structures::footballer::Footballer;



mod structures{
    pub mod footballer;

}
fn main (){

 let mut team1 :Vec<Footballer> = Vec::new();
 let mut team2 :Vec<Footballer> = Vec::new();

    for i in 0..11{
        let mut footballer1 = Footballer::new(String::from("suave"), String::from("suave"), 1, String::from("goalkepper"), String::from("deportivo chacarita"), 1.0
        );
        team1.push(footballer1);


        println!("Name: {}, Lastname: {}, Age: {}, Position: {}, Club: {}, Media: {}", team1[i].get_name(), team1[i].get_lastname(), team1[i].get_age(), team1[i].get_position(), team1[i].get_club(), team1[i].get_media());

    }


}