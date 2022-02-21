// Arrays - Fixed list where elements are the same data types

use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Re-assign a value
    numbers[0] = 21;

    // Get Array length
    println!("Length of array: {}", numbers.len());

    // Printing multiple numbers at once
    println!("{:?}", numbers); // need the debug :?

    // Printing single number
    println!("Single Value: {}", numbers[0]);
    
    // Arrays are stack allocated(will import inbuilt library)
    println!("Array is {} bytes", mem::size_of_val(&numbers)); // we pass the reference in the size_of_var and not the entire array, referene is address of first element of the array

    // Get slice
    let slice: &[i32] = &numbers;
    let slice1: &[i32] = &numbers[0..2]; // slicing till index 1, not including 2

    println!("Slice {:?}", slice);
    println!("Slice1 {:?}", slice1);
}
