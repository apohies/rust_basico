pub struct Footballer {
    pub name: String,
    pub lastname: String,
    pub age: i32,
    pub position: String,
    pub club: String,
    pub media : f32
}


impl Footballer {
    pub fn new(name: String, lastname: String, age: i32, position: String, club: String , media : f32) -> Footballer {
        Footballer {
            name,
            lastname,
            age,
            position,
            club,
            media
        }
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_lastname(&self) -> &String {
        &self.lastname
    }
    pub fn get_age(&self) -> i32 {
        self.age
    }
    pub fn get_position(&self) -> &String {
        &self.position
    }
    pub fn get_club(&self) -> &String {
        &self.club
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_lastname(&mut self, lastname: String) {
        self.lastname = lastname;
    }
    pub fn set_age(&mut self, age: i32) {
        self.age = age;
    }
    pub fn set_position(&mut self, position: String) {
        self.position = position;
    }
    pub fn set_club(&mut self, club: String) {
        self.club = club;
    }

    pub fn get_media(&self) -> f32 {
        self.media
    }
    pub fn set_media(&mut self, media: f32) {
        self.media = media;
    }
}