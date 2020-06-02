// Using a constructor pattern
// To run: cargo run --bin constructor

fn main() {
    let name_length = NameLength::new("Micheal");
    name_length.print();
}

struct NameLength {
    name: String,
    length: usize,
}

impl NameLength {
    // user doesn't need to setup length explicitly. Provided a method named 'new' that returns Self
    fn new(name: &str) -> Self {
        NameLength {
            length: name.len(),
            name: name.to_string(),
        }
    }

    fn print(&self) {
        println!("The name is '{}' is '{}' characters long", self.name, self.length);
    }
}

