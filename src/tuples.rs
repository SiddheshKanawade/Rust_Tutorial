// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    let person: (&str, &str, i8) = ("Brad", "Mass", 37);

    println!("{} is from {} and is {}", person.0, person.1, person.2);
}
