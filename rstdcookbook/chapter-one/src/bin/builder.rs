// Using the builder pattern
// To run: cargo run --bin builder

// uncomment the following to suppress unused_variables and dead_code warnings
// #![allow(unused_variables)]
// #![allow(dead_code)]

fn main() {
    println!(">>> Builder pattern demo");

    let normal_burger = BurgerBuilder::new().build();
    let cheese_burger = BurgerBuilder::new().cheese(true).salad(false).build();
    let veggie_bigmac = BurgerBuilder::new().vegetarian(true).patty_count(2).build();

    if let Ok(normal_burger) = normal_burger {
        normal_burger.print();
    }

    if let Ok(cheese_burger) = cheese_burger {
        cheese_burger.print();
    }

    if let Ok(veggie_bigmac) = veggie_bigmac {
        veggie_bigmac.print();
    }

    // constructing invalid burger
    let invalid_burger = BurgerBuilder::new().vegetarian(true).bacon(true).build();
    if let Err(error) = invalid_burger {
        println!("Failed to print burger: {}", error);
    }

    // reusing the builder to build multiple burgers
    let cheese_burger_builder = BurgerBuilder::new().cheese(true);
    for i in 1..10 {
        let cheese_burger = cheese_burger_builder.build();
        if let Ok(cheese_burger) = cheese_burger {
            println!("Cheese burger number {} is ready!", i);
            cheese_burger.print();
        }
    }

}

// structure definition for a configurable object
struct Burger {
    patty_count: i32,
    vegetarian: bool,
    cheese: bool,
    bacon: bool,
    salad: bool,
}

impl Burger {
    fn print(&self) {
        let pretty_patties = if self.patty_count == 1 {
            "patty"
        }
        else {
            "patties"
        };

        let pretty_bool = |val| if val {""} else { "no" };

        let pretty_vegetarian = if self.vegetarian { "vegeterian"} else { "" };

        println!(
            "This is a {} burger with {} {}, {} cheese, {} bacon and {} salad",
            pretty_vegetarian,
            self.patty_count,
            pretty_patties,
            pretty_bool(self.cheese),
            pretty_bool(self.bacon),
            pretty_bool(self.salad)
        );
    }
}

// Builder for Burger
struct BurgerBuilder {
    patty_count: i32,
    vegetarian: bool,
    cheese: bool,
    bacon: bool,
    salad: bool,
}

impl BurgerBuilder {
    fn new() -> Self {
        BurgerBuilder {
            patty_count: 1,
            vegetarian: false,
            cheese: false,
            bacon: false,
            salad: true,
        }
    }

    // define methods for each configurable value
    fn patty_count(mut self, val: i32) -> Self {
        self.patty_count = val;
        self
    }

    fn vegetarian(mut self, val: bool) -> Self {
        self.vegetarian = val;
        self
    }

    fn cheese(mut self, val: bool) -> Self {
        self.cheese = val;
        self
    }

    fn bacon(mut self, val: bool) -> Self {
        self.bacon = val;
        self
    }

    fn salad(mut self, val: bool) -> Self {
        self.salad = val;
        self
    }

    // actual builder method to construct an object
    fn build(&self) -> Result<Burger, String> {
        let burger = Burger {
            patty_count: self.patty_count,
            vegetarian: self.vegetarian,
            cheese: self.cheese,
            bacon: self.bacon,
            salad: self.salad,
        };

        // check for invalid configuration
        if burger.vegetarian && burger.bacon {
            Err("Sorry, no vegetarian bacon burger available.".to_string())
        }
        else {
            Ok(burger)
        }
    }
}
