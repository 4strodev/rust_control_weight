use features::person::domain::person::Person;
use std::{
    collections::HashMap,
    fmt::Debug,
    io::{self, Write},
    str::FromStr,
};

pub mod features;

fn main() {
    let mut neighborhoods: HashMap<String, Vec<Person>> = HashMap::new();
    let mut neighborhoods_avg: HashMap<String, f32> = HashMap::new();

    loop {
        // Asking for a neighborhood
        let (neighborhood, persons) = ask_neighborhood();

        if neighborhood == "" {
            break;
        }

        // Saving results in hashmap
        neighborhoods.insert(neighborhood.to_string(), persons);

        // Getting avg
        neighborhoods.iter().for_each(|persons| {
            let persons = persons.1;

            // Getting avg of neighborhood
            let avg = match persons
                .iter()
                // Mapping persons to their index
                .map(|person| person.get_index())
                // Getting the total of indexos
                .reduce(|acum, index| acum + index)
            {
                // If reduce return a value then calc the index
                Some(value) => value / persons.len() as f32,
                // else return 0
                None => 0f32,
            };

            // Saving neighborhood average
            neighborhoods_avg.insert(neighborhood.to_string(), avg);
        });
    }

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
        .expect("Error reading from stdin!");

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
            Err(_) => println!("Error trying to convert input!"),
        }
    }
}

/// Ask for a neighborhood to the user and return their name and the people inserted in that
/// neighborhood
fn ask_neighborhood() -> (String, Vec<Person>) {
    let neighborhood = input("Put the neighborhood: ");
    let mut persons: Vec<Person> = vec![];
    // If no neighborhood introduced then return;
    if neighborhood == "" {
        return (neighborhood, persons);
    }

    // Asking for persons
    loop {
        let name = input("Put the name: ");
        // If name is empty break loop
        if name == "" {
            break;
        }

        let person: Person = Person {
            name,
            age: input_as("Put the age: "),
            height: input_as("Put the height: "),
            weight: input_as("Put the weight: "),
        };

        persons.push(person);
    }

    (neighborhood, persons)
}
