// Providing a default implementation
// To run: cargo run --bin default

fn main() {
    println!(">>> default implementation test");

    let foo: i32 = Default::default();
    println!("foo: {}", foo);

    // a struct that derives from Default can be initialized like
    let pizza: PizzaConfig = Default::default();
    println!("wants_cheese: {}", pizza.wants_cheese);
    println!("number_of_olives: {}", pizza.number_of_olives);
    println!("special_message: {}", pizza.special_message);

    let crust_type = match pizza.crust_type {
        CrustType::Thin => "Nice and thin",
        CrustType::Thick => "Extra thick and extra filling"
    };
    println!("crust_type: {}", crust_type);

    // configuring only certain values
    let custom_pizza = PizzaConfig {
        number_of_olives: 12,
        crust_type: CrustType::Thin,
        ..Default::default() // leave other parameters default
    };
    println!("Custom pizza --> wants_cheese: {}", custom_pizza.wants_cheese);
    println!("Custom pizza --> number_of_olives: {}", custom_pizza.number_of_olives);
    println!("Custom pizza --> special_message: {}", custom_pizza.special_message);
}

# [derive(Default)]
struct PizzaConfig {
    wants_cheese: bool,
    number_of_olives: i32,
    special_message: String,
    crust_type: CrustType,
}

enum CrustType {
    Thin,
    Thick,
}

impl Default for CrustType {
    fn default() -> CrustType {
        CrustType::Thick
    }
}