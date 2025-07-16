//! # daqhats-rs
//!
//! Rust bindings for the MCC DAQ HAT Library, providing access to Measurement Computing
//! Corporation's data acquisition HAT devices for Raspberry Pi.
//!
//! This library provides safe Rust bindings to the C library for MCC DAQ HAT devices including:
//! - MCC 118: 8-Channel Analog Input HAT
//! - MCC 128: 8-Channel Analog Input HAT with thermocouple support
//! - MCC 134: 4-Channel Thermocouple Input HAT
//! - MCC 152: 2-Channel Analog Output / 8-Channel Digital I/O HAT
//! - MCC 172: 2-Channel 24-bit Sigma-Delta A/D HAT
//!
//! ## Platform Support
//!
//! This library is designed to work on Raspberry Pi systems with the appropriate MCC DAQ HAT
//! hardware installed. The underlying C library requires GPIO access and specific hardware
//! drivers that are only available on Raspberry Pi.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use daqhats_rs::*;
//!
//! // List all connected DAQ HATs
//! let count = unsafe { hat_list(HatIDs_HAT_ID_ANY as u16, std::ptr::null_mut()) };
//! println!("Found {} DAQ HAT devices", count);
//!
//! // Check for MCC 118 devices specifically
//! let mcc118_count = unsafe { hat_list(HatIDs_HAT_ID_MCC_118 as u16, std::ptr::null_mut()) };
//! if mcc118_count > 0 {
//!     println!("Found {} MCC 118 devices", mcc118_count);
//! }
//! ```
//!
//! ## Safety
//!
//! This library provides direct bindings to the C library, so most functions are marked as `unsafe`.
//! Users should be familiar with the MCC DAQ HAT C library documentation and ensure proper
//! error handling when using these functions.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// Result type for DAQ HAT operations
pub type DaqResult<T> = Result<T, i32>;

/// Convert a C result code to a Rust Result
pub fn check_result(result: i32) -> DaqResult<()> {
    if result == ResultCode_RESULT_SUCCESS {
        Ok(())
    } else {
        Err(result)
    }
}

/// Get a human-readable error message for a result code
/// 
/// Note: This function requires the C library to be linked and will only work on Raspberry Pi
/// systems with the DAQ HAT library installed.
#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
pub fn error_message(result: i32) -> String {
    unsafe {
        let msg_ptr = hat_error_message(result);
        if msg_ptr.is_null() {
            format!("Unknown error code: {}", result)
        } else {
            std::ffi::CStr::from_ptr(msg_ptr)
                .to_string_lossy()
                .into_owned()
        }
    }
}

/// Get a human-readable error message for a result code (fallback for non-ARM systems)
#[cfg(not(any(target_arch = "arm", target_arch = "aarch64")))]
pub fn error_message(result: i32) -> String {
    match result {
        0 => "Success, no errors".to_string(),
        -1 => "A parameter passed to the function was incorrect".to_string(),
        -2 => "The device is busy".to_string(),
        -3 => "There was a timeout accessing a resource".to_string(),
        -4 => "There was a timeout while obtaining a resource lock".to_string(),
        -5 => "The device at the specified address is not the correct type".to_string(),
        -6 => "A needed resource was not available".to_string(),
        -7 => "Could not communicate with the device".to_string(),
        -10 => "Some other error occurred".to_string(),
        _ => format!("Unknown error code: {}", result),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(ResultCode_RESULT_SUCCESS, 0);
        assert_eq!(HatIDs_HAT_ID_MCC_118, 322);
        assert_eq!(MAX_NUMBER_HATS, 8);
    }

    #[test]
    fn test_error_message() {
        let msg = error_message(ResultCode_RESULT_SUCCESS);
        assert!(!msg.is_empty());
        assert!(msg.contains("Success") || msg.contains("success"));
    }

    #[test]
    fn test_check_result() {
        assert!(check_result(ResultCode_RESULT_SUCCESS).is_ok());
        assert!(check_result(ResultCode_RESULT_BAD_PARAMETER).is_err());
    }
}