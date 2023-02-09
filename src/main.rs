use features::person::domain::person::Person;
use std::{
    collections::HashMap,
    fmt::Debug,
    io::{self, Write},
    process::exit,
    str::FromStr,
};

pub mod features;

fn main() {
    let mut neighborhoods: HashMap<&str, Vec<Person>> = HashMap::new();
    let mut neighborhoods_avg: HashMap<&str, f32> = HashMap::new();

    let (neighborhood, persons) = ask_neighborhood();

    neighborhoods.insert(&neighborhood, persons);

    // Getting avg
    neighborhoods.iter().for_each(|persons| {
        let persons = persons.1;

        // Getting avg of neighborhood
        let avg = match persons
            .iter()
            // Mapping persons to their index
            .map(|person| person.weight / (person.height * person.height))
            // Getting the total of indexos
            .reduce(|acum, index| acum + index)
        {
            // If reduce return a value then calc the index
            Some(value) => value / persons.len() as f32,
            // else return 0
            None => 0f32,
        };

        // Saving neighborhood average
        neighborhoods_avg.insert(&neighborhood, avg);
    });

    println!("{:?}", neighborhoods_avg);
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

    input.trim().to_string()
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

fn ask_neighborhood() -> (String, Vec<Person>) {
    let neighborhood = input("Introdueix el barri: ");
    // If no neighborhood introduced then stop program;
    if neighborhood == "" {
        exit(0);
    }
    let mut persons: Vec<Person> = vec![];
    loop {
        let name = input("Introdueix el nom: ");
        // If name is empty break loop
        if name == "" {
            break;
        }

        let person: Person = Person {
            name,
            age: input_as("Introdueix l'edat: "),
            height: input_as("Introdueix l'alçada: "),
            weight: input_as("Introdueix el pes: "),
        };

        persons.push(person);
    }

    (neighborhood, persons)
}
