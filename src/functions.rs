// Functions - Used to store blocks of code for re-use

pub fn run() {
    greeting("Hello", "Jane");
    
    // Bind function values to variables
    let y = 23;
    let x: i32 = add(12, y);
    println!("Sum: {}", x);

    // Closure
    let n3: i32 = 34;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3; // here I was allowed to add n3 which isnt defined in function block, this is not valid in case of the external functions.
    println!("AddNums: {}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

// Function with return
fn add(n1: i32, n2: i32) -> i32 {
    n1+n2 //without semicolon means return
}
