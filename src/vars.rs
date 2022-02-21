// Variables hold primitive data or reference to data
// Variables are immutable by default, it means we can't reassign it
// Rust is a block-scoped language

pub fn run(){
    let name = "Someone";
    let mut age = 20; // mut = makes variable mutable

    println!("There is {} in group whose age is {}", name, age);

    age = 12;

    println!("age is changed to {}", age);

    // Define constant
    // they require datatype to be explicitly mentioned
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Someone", 32);
    println!("{} is {}", my_name, my_age);
}