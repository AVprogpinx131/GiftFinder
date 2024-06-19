#[derive(Debug)]
pub enum Gender {
    Male, 
    Female,
}

#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: u8,
    pub gender: Gender,
    pub hobbies: Vec<String>,
}

impl Person {
    // Associated function (constructor)
    pub fn new(name: String, age: u8, gender: Gender, hobbies: Vec<String>) -> Person {
        Self { name, age, gender, hobbies }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn set_age(&mut self, age: u8) {
        self.age = age
    }

    pub fn add_hobby(&mut self, hobby: String) {
        self.hobbies.push(hobby);
    }

    pub fn remove_hobby(&mut self, hobby: &str) {
        self.hobbies.retain(|h| h != hobby);
    }

    pub fn hobby_count(&self) -> usize {
        self.hobbies.len()
    }
}