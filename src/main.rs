mod misc;
mod examples;
mod minigrep;
use std::{env, fmt::{Display, Error, Formatter, Result}};
use minigrep::{Config, run };

trait Describable {
    fn describe(&self) -> String;
}

struct Person {
    name: String,
    age: u8,
    height: f64,
    weight: f64
}

impl Describable for Person {
    fn describe(&self) -> String {
        format!("Person: {} of age {} and height {} weighs {}", self.name, self.age, self.height, self.weight)
    }
}

struct Book {
    name: String,
    pages: u32,
}

impl Describable for Book {
    fn describe(&self) -> String {
        format!("Book: {}, has {} pages", self.name, self.pages)
    }
}

impl Display for Book {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result { 
        write!(f, "Book {} has {} pages", self.name, self.pages)
    }
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Person {} is {} years old", self.name, self.age)
    }
}

fn print_description<T: Describable>(item: &T) {
    println!("Describing object: {}", item.describe());
}

fn describe_and_display<T: Describable + Display>(item: &T) {
    println!("{}", item.describe());
    println!("{}", item);
}

fn describe_both<T>(a: &T, b: &impl Describable) 
where T: Describable {
    println!("{}", a.describe());
    println!("{}", b.describe());
}

fn create_person() -> impl Describable {
    let person: Person = Person {name: String::from("Alec"), age: 32, height: 1.73, weight: 56.0 };
    person
}

struct Garment {
    kind: String,
    gender: String
}

impl Describable for Garment {
    fn describe(&self) -> String {
        return format!("Garment: {} for {}", self.kind, self.gender);
    }
}

// fn main() {
//     let book = Book { name: String::from("Harold fry"), pages: 500};
//     let person = Person { name: String::from("manish"), age: 30, height: 1.73, weight: 52.4};
//     let person2 = create_person();
//     print_description(&book);
//     print_description(&person);
//     describe_both(&person, &person2);
//     let contents = misc::read_from_file("some.md");
//     println!("{contents}");
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    let config_result= Config::build(&args);
    let content_result = run(config_result.unwrap()).unwrap();
}