// Primitive str = Immutable fixed-length string somewhere in memory, same datatype(char)
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data


pub fn run(){
    let mut hello = String::from("Hello ");


    // Getting length of the string
    println!("{}", hello.len());

    // Adding/pushing a unicode char
    hello.push('a');

    // Pushing a string
    hello.push_str("nne"); //single quotes means unicode, for string => double quotes

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Check empty
    println!("Is_Empty? {}", hello.is_empty());

    // Contains a substring
    println!("Contains world? {}", hello.contains("world"));

    // Replace 
    println!("Replace: {}", hello.replace("anne", "World!"));

    // Loop through string by whitespace
    for item in hello.split_whitespace(){
        println!("{}", item);
    }

    // Create string with capacity 
    let mut s = String::with_capacity(10);

    s.push('a');
    s.push('d');

    // Assertion testing(it matches right and left conditions and if both dont matcht then give error else it runs smoothly)
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
    assert_eq!(11, s.capacity());

    println!("{}", hello);
    println!("{}", s);

}