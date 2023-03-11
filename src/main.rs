use bit_vec::BitVec; // Bit vector for optimal memory use
use csv::Writer; // Input/Output reader // CSV file writer
use env_logger; // Basic logger
use std::f64::consts::PI; // Pi const value
use std::fs::OpenOptions; //Filesystem
use std::io;

#[macro_use]
extern crate log;

// Main function
fn main() {
    // Initialize the logger
    env_logger::init();

    // Request the input values for the QPSK modulation
    let sampling_freq = read_number("Sampling Frequency (Hz): ");
    let modulation_freq = read_number("Modulation Frequency (Hz): ");
    let simb_freq = read_number("Sampling Frequency (sinbol/s): ");
    let bit_data = read_even_bit_array_from_console("Enter input data stream: ");

    // Get period from frequencies
    let x: f64 = 1.0;
    let simb_tem: f64 = x / (f64::from(simb_freq)).powi(2);
    info!("simb_tem created: {}", simb_tem);
    let modulation_tem: f64 = x / (f64::from(modulation_freq));
    info!("modulation_tem created: {}", modulation_tem);
    let sampling_tem: f64 = x / (f64::from(sampling_freq));
    info!("sampling_tem created: {}", sampling_tem);

    // Even bit demoultiplex input data
    let (odd_bits, even_bits) = demultiplexor_even(bit_data);

    // Bits to NRZ signal
    let odd_sig = nrz_encoder(odd_bits.clone(), f64::from(1));
    let even_sig = nrz_encoder(even_bits.clone(), f64::from(1));

    let ampli = f64::sqrt(2.0 / simb_tem);
    let phi1 = create_phi(false, ampli, sampling_freq, f64::from(modulation_freq));
    let phi2 = create_phi(true, ampli, sampling_freq, f64::from(modulation_freq));

    // Write value in a CSV
    save_in_csv("test.csv", odd_bits);
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

fn read_even_bit_array_from_console(parameter: &str) -> BitVec {
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

        let mut bits: BitVec = BitVec::new();

        for c in input.chars() {
            if c == '1' {
                bits.push(true);
            } else if c == '0' {
                bits.push(false);
            }
        }
        info!("created bit array:{:?}", bits);

        //Format the input data to an array of bits

        // Check if the entered bit amount is even
        if bits.len() % 2 == 0 {
            info!("{} parameter returned:{:?}", parameter, bits);
            return bits;
        } else {
            error!("Entered bit array doesn't have an even number of bits");
            println!("The bit array must have an even number of bits. Please try again.");
        }
    }
}

/// Save info into a csv file
///
/// # Arguments
///  -`file_name`(&str): Name of the file to be used
///
fn save_in_csv(file_name: &str, values: BitVec) {
    //TODO: Add file path checker
    //Creates a new file if it doesn't already exist
    let file = OpenOptions::new()
        .write(true)
        .create(true) // This will create the file if it does not exist
        .open(file_name)
        .unwrap();

    // Create a CSV writer from the file
    let mut wtr = Writer::from_writer(file);

    let values_string: String = values
        .iter()
        .map(|bit| if bit { '1' } else { '0' })
        .collect();
    // Write some records to the CSV file
    wtr.write_record(&["name", &values_string]).unwrap();

    // Flush the writer to ensure everything is written
    wtr.flush().unwrap();
}

/// Even bit demultiplexor for bit array
///
fn demultiplexor_even(data: BitVec) -> (BitVec, BitVec) {
    let mut odd_data = BitVec::new();
    let mut even_data = BitVec::new();

    // Go through every single bit in the input array
    for (i, bit) in data.iter().enumerate() {
        // Check if the position of the bit is even or odd, i starts at 0
        if i % 2 == 0 {
            // Bit position is odd
            odd_data.push(bit);
        } else {
            // Bit position is even
            even_data.push(bit);
        }
    }
    info!(
        "Input:{:?}  Odd bit array:{:?} Even bit array:{:?} ",
        data, odd_data, even_data
    );

    (odd_data, even_data)
}

/// NRZ Encoder
/// Transforms 1 to sqrt(Eb) and 0 to -sqrt(Eb)

fn nrz_encoder(data: BitVec, eb: f64) -> Vec<f64> {
    let eb_sqrt = f64::sqrt(eb);
    let mut result = Vec::new();

    for (_i, bit) in data.iter().enumerate() {
        if bit {
            result.push(eb_sqrt);
        } else {
            result.push(-eb_sqrt);
        }
    }
    info!("NRZ Encoder - Input: {:?} - Output: {:?}", data, result);
    result
}

fn create_phi(sin: bool, amplitude: f64, fs: u32, fc: f64) -> Vec<f64> {
    let mut phi = Vec::new();
    let phase = 2.0 * PI * fc;
    if sin {
        for _i in 0..(fs as usize) {
            phi.push(amplitude * phase.sin());
        }
        return phi;
    }

    for _i in 0..(fs as usize) {
        phi.push(amplitude * phase.cos());
    }
    return phi;
}
