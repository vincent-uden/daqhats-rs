# daqhats-rs

[![Crates.io](https://img.shields.io/crates/v/daqhats-rs.svg)](https://crates.io/crates/daqhats-rs)
[![Documentation](https://docs.rs/daqhats-rs/badge.svg)](https://docs.rs/daqhats-rs)
[![License](https://img.shields.io/crates/l/daqhats-rs.svg)](https://github.com/yourusername/daqhats-rs#license)

Rust bindings for the [MCC DAQ HAT Library](https://github.com/mccdaq/daqhats), providing access to Measurement Computing Corporation's data acquisition HAT devices for Raspberry Pi.

## Supported Devices

- **MCC 118**: 8-Channel Analog Input HAT
- **MCC 128**: 8-Channel Analog Input HAT with thermocouple support  
- **MCC 134**: 4-Channel Thermocouple Input HAT
- **MCC 152**: 2-Channel Analog Output / 8-Channel Digital I/O HAT
- **MCC 172**: 2-Channel 24-bit Sigma-Delta A/D HAT

## Platform Requirements

This library is designed for **Raspberry Pi** systems with MCC DAQ HAT hardware installed. The underlying C library requires:
- Raspberry Pi OS (or compatible Linux distribution)
- GPIO access and hardware drivers
- `libgpiod` library

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
daqhats-rs = "0.1"
```

## Usage

```rust
use daqhats_rs::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // List all connected DAQ HATs
    let count = unsafe { hat_list(HatIDs_HAT_ID_ANY, std::ptr::null_mut()) };
    println!("Found {} DAQ HAT devices", count);

    if count > 0 {
        // Allocate memory for device info
        let mut devices = vec![HatInfo::default(); count as usize];
        unsafe {
            hat_list(HatIDs_HAT_ID_ANY, devices.as_mut_ptr());
        }

        for device in devices {
            println!("Device at address {}: ID = {}", device.address, device.id);
        }
    }

    Ok(())
}
```

## Safety

This library provides direct bindings to the C library, so most functions are marked as `unsafe`. Users should:
- Be familiar with the [MCC DAQ HAT C library documentation](https://mccdaq.github.io/daqhats/)
- Implement proper error handling
- Follow the C library's usage patterns and constraints

## Building

The library will automatically:
1. Generate Rust bindings from C headers using `bindgen`
2. On Raspberry Pi (ARM/AArch64): Compile the C library and link against it
3. On other platforms: Generate bindings only (for development)

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
