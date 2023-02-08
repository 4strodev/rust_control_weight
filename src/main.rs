use features::person::domain::person::Person;
use std::collections::HashMap;
use std::fmt::Debug;
use std::io;
use std::io::Write;
use std::str::FromStr;

pub mod features;

fn main() {
    let mut neighborhoods_avg: HashMap<&str, Vec<Person>> = HashMap::new();
    let neighborhood = input("Introdueix el barri: ").trim().to_string();
    if neighborhood == "" {
        std::process::exit(0);
    }

    let mut persons: Vec<Person> = vec![];
    loop {
        let name = input("Introdueix el nom: ").trim().to_string();
        // If name is empty break loop
        if name == "" {
            break;
        }
        let age: i8 = input_as("Introdueix l'edat: ");
        let height: f32 = input_as("Introdueix l'alçada: ");
        let weight: f32 = input_as("Introdueix el pes: ");

        let person: Person = Person {
            age,
            name,
            height,
            weight,
        };

        persons.push(person);
    }

    neighborhoods_avg.insert(&neighborhood, persons);

    neighborhoods_avg
        .iter()
        .for_each(|persons| println!("{:?}", persons.1));
}

/// Prints a message without \n character and returns stdin input
fn input(message: &str) -> String {
    let mut input = String::new();

    // Stdout is buffered this is why we are flushing buffer and ensuring that message is showed
    // before guessing something
    print!("{}", message);
    // Flushing stdout buffer
    io::stdout().flush().expect("Error flushing stdout!");

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading from stdin");

    input
}

/// Ask user for an input and converts it to specified data type
fn input_as<T: FromStr>(message: &str) -> T
where
    <T as FromStr>::Err: Debug,
{
    loop {
        match input(message).trim().parse::<T>() {
            Ok(number) => return number,
            Err(_) => println!("Error trying to convert input"),
        }
    }
}
