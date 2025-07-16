//! MCC 128 Single Value Read Example
//! 
//! This example demonstrates:
//! - mcc128_a_in_read
//! - mcc128_a_in_mode_write  
//! - mcc128_a_in_range_write
//!
//! Purpose:
//! Read a single data value for each channel in a loop.
//!
//! Description:
//! This example demonstrates acquiring data using a software timed loop
//! to read a single value from each selected channel on each iteration
//! of the loop.

use daqhats_rs::*;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

// Constants from the C library (these would be defined in the bindings)
const A_IN_MODE_SE: u8 = 0;  // Single-ended mode
const A_IN_RANGE_BIP_10V: u8 = 0;  // ±10V range

// Custom error type for the example
#[derive(Debug)]
struct ExampleError(String);

impl std::fmt::Display for ExampleError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ExampleError {}

impl From<i32> for ExampleError {
    fn from(code: i32) -> Self {
        ExampleError(error_message(code))
    }
}

impl From<std::io::Error> for ExampleError {
    fn from(err: std::io::Error) -> Self {
        ExampleError(err.to_string())
    }
}

fn main() -> Result<(), ExampleError> {
    // Configuration
    let low_chan = 0u8;
    let high_chan = 3u8;
    let sample_interval = Duration::from_millis(500);
    let input_mode = A_IN_MODE_SE;
    let input_range = A_IN_RANGE_BIP_10V;
    let options = OPTS_DEFAULT;

    println!("MCC 128 single data value read example");
    println!("    Functions demonstrated:");
    println!("        mcc128_a_in_read");
    println!("        mcc128_a_in_mode_write");
    println!("        mcc128_a_in_range_write");
    println!("    Input mode: Single-ended");
    println!("    Input range: ±10V");
    println!("    Channels: {} - {}", low_chan, high_chan);
    println!("    Options: Default");

    // Find MCC 128 device
    let address = find_mcc128_device()?;
    println!("Using MCC 128 at address {}", address);

    // Open connection to device
    let result = unsafe { mcc128_open(address) };
    check_result(result).map_err(ExampleError::from)?;
    println!("Device opened successfully");

    // Configure input mode
    let result = unsafe { mcc128_a_in_mode_write(address, input_mode) };
    check_result(result).map_err(ExampleError::from)?;

    // Configure input range  
    let result = unsafe { mcc128_a_in_range_write(address, input_range) };
    check_result(result).map_err(ExampleError::from)?;

    println!("\nPress Enter to continue...");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    println!("Acquiring data... Press Ctrl+C to stop\n");

    // Display header
    print!("  Samples/Channel");
    for channel in low_chan..=high_chan {
        print!("     Channel {}", channel);
    }
    println!();

    let mut samples_per_channel = 0;

    // Main acquisition loop
    loop {
        samples_per_channel += 1;
        print!("\r{:17}", samples_per_channel);

        // Read from each channel
        for channel in low_chan..=high_chan {
            let mut value = 0.0f64;
            let result = unsafe { 
                mcc128_a_in_read(address, channel, options, &mut value as *mut f64) 
            };
            
            match check_result(result) {
                Ok(_) => print!("{:12.5} V", value),
                Err(e) => {
                    eprintln!("\nError reading channel {}: {}", channel, error_message(e));
                    break;
                }
            }
        }

        io::stdout().flush()?;
        thread::sleep(sample_interval);
    }

    // Cleanup (this won't be reached due to Ctrl+C, but good practice)
    #[allow(unreachable_code)]
    {
        let result = unsafe { mcc128_close(address) };
        if let Err(e) = check_result(result) {
            eprintln!("Error closing device: {}", error_message(e));
        }
        Ok(())
    }
}

/// Find the first MCC 128 device
fn find_mcc128_device() -> Result<u8, ExampleError> {
    // Get count of MCC 128 devices
    let count = unsafe { hat_list(HatIDs_HAT_ID_MCC_128 as u16, std::ptr::null_mut()) };
    
    if count <= 0 {
        return Err(ExampleError("No MCC 128 devices found. Make sure the device is connected and recognized.".to_string()));
    }

    // Get device info
    let mut devices = vec![create_default_hat_info(); count as usize];
    unsafe {
        hat_list(HatIDs_HAT_ID_MCC_128 as u16, devices.as_mut_ptr());
    }

    // Return address of first device
    Ok(devices[0].address)
}

// Helper function to create a default HatInfo
fn create_default_hat_info() -> HatInfo {
    HatInfo {
        address: 0,
        id: 0,
        version: 0,
        product_name: [0; 256],
    }
}