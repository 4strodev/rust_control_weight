use std::fmt;

#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: i8,
    pub weight: f32,
    pub height: f32,
}

/// Converts struct fiels into string representation
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Name: {}\nAge: {}\nWeight{}\nHeight: {}",
            self.name, self.age, self.weight, self.height
        )
    }
}
