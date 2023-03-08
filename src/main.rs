use env_logger;
use std::io;

#[macro_use]
extern crate log;

// Main function
fn main() {
    // Initialize the logger
    env_logger::init();

    // Request the input values for the QPSK modulation
    let _sampling_freq = read_number("Sampling Frequency (Hz): ");
    let _modulation_freq = read_number("Modulation Frequency (Hz): ");
    let _simb_freq = read_number("Sampling Frequency (sinbol/s): ");
    let _bit_data = read_even_bit_array_from_console("Enter input data stream: ");

    //
}

///  Returns the number that the user entered via the console line
///
/// # Arguments
/// * `parameter`(&str) - It will show the parameter to the user as a request
/// # Return
///  Unsigned Integer (u32) with the values the user entered
///  # Example
/// ```
/// let number = read_number("Test")
/// '''

fn read_number(parameter: &str) -> u32 {
    // Show the user the
    println!("Enter {}", parameter);
    let mut input_string = String::new(); // Create a mutable string variable

    io::stdin()
        .read_line(&mut input_string) // call read_line with a mutable reference to input
        .expect("Failed to load read input "); // handle errors
    let input_number: u32 = input_string
        .trim()
        .parse() //call trim and parse on the input and specify i32 type
        .expect("Invalid number, enter an unsigned nummber "); //handle errors

    //TODO: Keep repeating until a positive number has been entered
    info!("{} parameter entered:{}", parameter, input_number);
    input_number
}

/// Keeps asking the user for a even number of input bits to process
///
/// # Arguments
/// * `parameter`(&str) - It will show the parameter to the user as a request
/// # Return
///  Bit vector (Vec<boll>) with the values the user entered
///  # Example
/// ```
/// let number = read_number("Test")
/// '''

fn read_even_bit_array_from_console(parameter: &str) -> Vec<bool> {
    // Keeps asking the user for a even number of bits
    loop {
        // Shows the name of the parameter that the user has to enter
        println!("{} (even bit array): ", parameter);

        let mut input = String::new(); // Create a mutable string variable

        // Read the console for data
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        info!("{} parameter entered:{}", parameter, input);

        //Format the input data to an array of bits
        let bits: Vec<bool> = input.trim().chars().map(|c| c == '1').collect();

        // Check if the entered bit amount is even
        if bits.len() % 2 == 0 {
            info!("{} parameter returned:{}", parameter, input);
            return bits;
        } else {
            error!("Entered bit array doesn't have an even number of bits");
            println!("The bit array must have an even number of bits. Please try again.");
        }
    }
}
