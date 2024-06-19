// use std::collections::HashSet;

pub fn vectors_operations() {
    // Declare a vector of integers
    let mut numbers_data = vec![1, 2, 3, 3, 4, 5, 6, 7, 8, 9, 10];

    // Access elements of the vector
    println!("Numbers data: {:?}", numbers_data); 
    // println!("Length of the vector: {}", numbers_data.len());
    println!("First number is {0} and last number is {1}", numbers_data[0], numbers_data[numbers_data.len() - 1]); 

    for number in &numbers_data[..3] {
        println!("Looping sliced elements: {:?}", number);
    }

    println!("Sliced elements: {:?}", &numbers_data[1..4]);

    // Add new element to the vector
    numbers_data.push(5);
    numbers_data.push(6);

    // Remove first and last element from the vector
    numbers_data.remove(0);
    numbers_data.pop();

    // Sort the vector
    numbers_data.sort();

    println!("Sorted numbers data: {:?}", numbers_data);

    // Remove consecutive duplicates from the vector
    numbers_data.dedup();

    println!("No duplicates vector: {:?}", numbers_data);

    // Remove non-consecutive duplicates from the vector
    // let set: HashSet<_> = numbers_data.into_iter().collect();
    // let mut vec: Vec<_> = set.into_iter().collect();
    // vec.sort_unstable();

    // Return min and max element from the vector
    let min = numbers_data.iter().min().unwrap();
    let max = numbers_data.iter().max().unwrap();

    println!("Min number is {} and max number is {}", min, max);

    // Find the sum of all elements in the vector
    let sum: i32 = numbers_data.iter().sum();
    println!("Sum of all elements in the vector: {}", sum);


}