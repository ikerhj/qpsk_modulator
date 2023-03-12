use bit_vec::BitVec; // Bit vector for optimal memory use
use csv::WriterBuilder; // Input/Output reader // CSV file writer
                        // Basic logger
use env_logger;
use plotters::prelude::*;
use std::error::Error;
use std::f64::consts::PI; // Pi const value
use std::fs::OpenOptions; //Filesystem
use std::io;
use std::io::BufWriter;

#[macro_use]
extern crate log;

// Main function
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger
    env_logger::init();

    // Request the input values for the QPSK modulation
    let sampling_freq = read_number("Sampling Frequency (Hz): ");
    let carrier_freq = read_number("Carrier Frequency (Hz): ");
    let symbol_rate = read_number("Symbol rate (sinbol/s): ");
    let bit_data = read_even_bit_array_from_console("Enter input data stream: ");

    // Create variables related to the signal that has to be created  from input values
    let bit_rate = (f64::from(symbol_rate)) * 2.0; // (bit/second)
    let symbol_period: f64 = 1.0 / f64::from(symbol_rate); //(seconds/bit)
    let duration = (bit_data.clone().len() as f64) / bit_rate; // Time needed for processing the input bitstream
    let sample_period = 1.0 / (sampling_freq as f64); //Duration of a sample;
    let samples = duration / sample_period; // Quantity of samples needed for processing the input bit array
    let samples_per_bit = samples / (bit_data.clone().len() as f64); // Quantity of samples used for processing a bit
    info!(
        "Time duration: {} - sample_duration: {} -Quantity of samples used :{} - symbol_period: {}",
        duration, sample_period, samples, symbol_period
    );

    // Even bit demoultiplex the input data
    let (odd_bits, even_bits) = even_demultiplexor(bit_data.clone());

    // Bits to NRZ signal
    let odd_sig = nrz_encoder(odd_bits.clone(), f64::from(1), samples_per_bit);
    let even_sig = nrz_encoder(even_bits.clone(), f64::from(1), samples_per_bit);

    // Generate phi signal that is used for multiplying it to NRZ encoded signal
    let time_space = create_time(0.0, duration, sample_period * 2.0);
    let amplitude = f64::sqrt(2.0 / symbol_period);
    let phi1 = phi_generator(
        false,
        amplitude,
        time_space.clone(),
        f64::from(carrier_freq),
    ); // Inphase element
    let phi2 = phi_generator(true, amplitude, time_space.clone(), f64::from(carrier_freq)); // Quadrature elements

    // Multiply the analog NRZ and phi carrier signals
    let inphase_elements = multiply_vectors(odd_sig, phi1);
    let quadrature_elements = multiply_vectors(even_sig, phi2);

    // Create the QPSK signal bi adding the inphase and quadrature signals
    let qpsk_signal = add_vectors(inphase_elements, quadrature_elements);

    // Write QPSK signal in a CSV
    if let Err(error) = save_qpsk_in_csv(
        "QPSK_signal_samples.csv",
        qpsk_signal.clone(),
        samples_per_bit as usize,
    ) {
        println!("Error when saving the signal samples in the file ./QPSK_signal_samples.csv . Check the logs for more info {}", error);
    };

    // Visualize data in a plot plot
    let bit_string = bit_to_string(bit_data.clone());
    let title = format!("{:?} input QPSK Modulated signal", bit_string);
    plot_signal(
        &qpsk_signal.clone(),
        &time_space.clone(),
        &title,
        "QPSK_signal_plot.png",
    )?;

    Ok(())
}

///  Returns the number that the user entered via the console line'

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

        // Goes through the string and creates the bit vector
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

/// Even bit demultiplexor for bit array
fn even_demultiplexor(data: BitVec) -> (BitVec, BitVec) {
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

/// Transforms 1 to sqrt(Eb) and 0 to -sqrt(Eb) and creates a signal of entered samples of bits
fn nrz_encoder(bit_stream: BitVec, eb: f64, samples_per_bit: f64) -> Vec<f64> {
    let eb_sqrt = f64::sqrt(eb);
    let mut encoded_signal = Vec::new();

    for (_i, bit) in bit_stream.iter().enumerate() {
        let amplitude = if bit { eb_sqrt } else { -eb_sqrt };
        for _i in 0..(samples_per_bit as usize) {
            encoded_signal.push(amplitude);
        }
    }
    info!(
        "NRZ Encoder - Input bitstream: {:?} -fs :{} - Output length: {:?}",
        bit_stream,
        samples_per_bit,
        encoded_signal.len()
    );
    encoded_signal
}

// Creates the queried phi carrier
fn phi_generator(sin: bool, amplitude: f64, mut time_space: Vec<f64>, fc: f64) -> Vec<f64> {
    let mut phi = Vec::new();
    let mut phase = Vec::new();
    // Multiply each element of the time array by the phase of the carrier
    for elem in time_space.iter_mut() {
        phase.push(*elem * 2.0 * PI * fc);
    }

    if sin {
        for elem in phase.iter_mut() {
            phi.push(amplitude * elem.sin());
        }
    } else {
        for elem in phase.iter_mut() {
            phi.push(amplitude * elem.cos());
        }
    }

    info!("Created phi value length: {}", phi.len());
    phi
}

// Creates time space for signal based on max, min and step size
fn create_time(min: f64, max: f64, step: f64) -> Vec<f64> {
    let mut time = Vec::new();
    let mut step_value = min;
    while step_value < max {
        time.push(step_value);
        step_value += step;
    }
    info!(
        "Created time  - Min: {} - Max: {} - Step: {}",
        min, max, step
    );
    time
}

// Adds two Vec<f64> vectors and returns the result
fn add_vectors(array1: Vec<f64>, array2: Vec<f64>) -> Vec<f64> {
    let mut sum_array = Vec::new();
    for (i, value) in array1.iter().enumerate() {
        sum_array.push(value + array2[i]);
    }
    info!("Array sum length: {}", sum_array.len());
    sum_array
}

// Multiplies two Vec<f64> vectors and returns the result
fn multiply_vectors(array1: Vec<f64>, array2: Vec<f64>) -> Vec<f64> {
    let mut sum_array = Vec::new();
    for (i, value) in array1.iter().enumerate() {
        sum_array.push(value * array2[i]);
    }
    info!("Array multiply length: {}", sum_array.len());
    sum_array
}

/// Creates a Sting from a bit vector input
fn bit_to_string(data: BitVec) -> String {
    let mut bit_string = String::new();
    for bit in data.clone() {
        if bit {
            bit_string.push('1');
        } else {
            bit_string.push('0');
        }
    }
    bit_string
}

/// Save signal by symbol/line into a csv file
fn save_qpsk_in_csv(
    file_name: &str,
    signal: Vec<f64>,
    samples_per_line: usize,
) -> Result<(), Box<dyn Error>> {
    let lines = signal.len() / samples_per_line;
    let mut divided_signal = vec![vec![0.0; samples_per_line]; lines];

    // Divide the signal into lines
    for i in 0..lines {
        for j in 0..samples_per_line {
            divided_signal[i][j] = signal[i * samples_per_line + j];
        }
    }

    //Creates a new file if it doesn't already exist
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_name)?;

    // Create a CSV writer from the file
    let mut writer = WriterBuilder::new()
        .has_headers(false)
        .from_writer(BufWriter::new(file));

    // Write some records to the CSV file
    for line in divided_signal {
        //Line of floats parsed to string
        let str_line: Vec<String> = line.iter().map(|f| f.to_string()).collect();
        writer.write_record(&str_line)?;
    }

    // Flush the writer to ensure everything is written
    writer.flush().unwrap();
    info!(
        "QPSK modulated signal samples saved by symbol/line in: {}",
        file_name
    );
    Ok(())
}

// Creates a .png image of a plot from a passed signal values
fn plot_signal(
    signal: &[f64],
    time: &[f64],
    title: &str,
    file_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(file_name, (1280, 720)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 20))
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(time[0]..time[time.len() - 1], -57.0..57.0)?;

    chart.configure_mesh().x_desc("time(s)").draw()?;

    chart.draw_series(LineSeries::new(
        time.iter().zip(signal.iter()).map(|(x, y)| (*x, *y)),
        &BLUE,
    ))?;

    info!("QPSK signal plot created at: {}", file_name);
    Ok(())
}
