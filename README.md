# EasyTime Library

The EasyTime library is a Rust crate that provides a simple and flexible way to perform date and time calculations with ease. It leverages the `chrono` library to handle date and time manipulations while allowing you to work with various time zones and perform operations like adding or subtracting seconds, minutes, hours, days, months, and years from the current time.

## Features

- Easy-to-use API for date and time calculations.
- Support for working with local time and UTC time.
- Precise handling of leap years in month and year calculations.
- Calculation methods for adding or subtracting seconds, minutes, hours, days, months, years, decades, centuries and milleniums.

## Installation

To use the EasyTime library in your Rust project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
easytime = "0.1.0"
```

## Usage

Here's a basic example of how to use the EasyTime library:

```
extern crate easytime;
use easytime::{EasyTime, Local};

fn main() {
    // Create an EasyTime instance with a value of 10 seconds.
    let easy_time = EasyTime::new(10);

    // Calculate the time 10 seconds from now.
    let future_time = easy_time.seconds_from_now();

    // Calculate the time 10 seconds ago.
    let past_time = easy_time.seconds_ago();

    println!("Future Time: {}", future_time);
    println!("Past Time: {}", past_time);
}
```
This example demonstrates how to create an EasyTime instance, add or subtract seconds from the current time, and print the results.

### Contributing
Contributions to the EasyTime library are welcome! Feel free to open issues, submit pull requests, or provide feedback to help improve this library.