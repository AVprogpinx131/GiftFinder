mod vectors;
mod person;

use crate::person::Person;
use crate::person::Gender;

fn main() {
    vectors::vectors_operations();

    let mut person = Person::new (
        String::from("Andrus Vaher"),
        20,
        Gender::Male,
        vec![String::from("Programming"), String::from("Reading"), String::from("Gaming")],
    );

    println!("Person: {:?}", person);

    person.set_age(21);
    person.add_hobby(String::from("Cycling"));
    person.remove_hobby("Gaming");

    println!("Updated Person: {:?}", person);
    println!("Number of Hobbies: {}", person.hobby_count());
}
