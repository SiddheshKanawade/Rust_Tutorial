// Vectors - Resizable arrays, can vary with size


use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Re-assign a value
    numbers[0] = 21;

    // Add on to Vector
    numbers.push(10);
    numbers.push(12);

    // Pop last value
    numbers.pop();
    // Get Array length
    println!("Length of vector: {}", numbers.len());

    // Printing multiple numbers at once
    println!("{:?}", numbers); // need the debug :?

    // Printing single number
    println!("Single Value: {}", numbers[0]);
    
    // Arrays are stack allocated(will import inbuilt library)
    println!("Vector is {} bytes", mem::size_of_val(&numbers)); // we pass the reference in the size_of_var and not the entire array, referene is address of first element of the array

    // Loop through vector values
    for x in numbers.iter(){
        println!("Number {}", x);
    }

    // Loop and mutate the values(somewhat similar to map in JS)
    for x in numbers.iter_mut(){
        *x *= 2;
    }

    // Get slice, this is a immutable borrow
    let slice: &[i32] = &numbers;
    let slice1: &[i32] = &numbers[0..2]; // slicing till index 1, not including 2


    println!("Slice {:?}", slice);
    println!("Slice1 {:?}", slice1);
}
