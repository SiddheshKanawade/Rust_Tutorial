pub fn run(){
    // Print to console
    println!("Print.rs");

    // Basic formating
    println!("{} came {}st in the class", "Someone", 1);

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

    // Named arguments
    println!("{name} likes to play {activity}", name = "John", activity = "Baseball");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 20 , 30);

    // Placeholder for debug traits
    println!("{:?}", (12, true, "Hello")); //tuple

    // Basic math
    println!("10 + 10 = {}", 10+10);
}