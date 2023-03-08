use std::io;

// Main function
fn main() {
    let sampling_freq = read_number();
    println!("The number is {}", sampling_freq);
}

//
fn read_number() -> i32 {
    println!("Enter Sampling Frequency");
    let mut input_string = String::new(); // Create a mutable string variable

    io::stdin()
        .read_line(&mut input_string) // call read_line with a mutable reference to input
        .expect("Failed to load read input"); // handle errors
    let input_number: i32 = input_string
        .trim()
        .parse() //call trim and parse on the input and specify i32 type
        .expect("Invalid number");
    //handle errors
    input_number
}
