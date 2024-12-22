
# EasyTime Library

The EasyTime library is a powerful Rust crate designed to simplify date and time calculations. It builds upon the `chrono` library to provide robust support for time zone-aware operations, leap year handling, and extensive formatting options. With EasyTime, you can easily add or subtract seconds, minutes, hours, days, months, years, decades, centuries, and millennia from the current time.

---

## Features

- **Time Zone Support**: Work with local time (`Local`) or UTC time (`Utc`).
- **Flexible Date-Time Offsets**: Add or subtract time intervals with methods for seconds, minutes, hours, days, months, years, and beyond.
- **Precise Calculations**: Handles leap years, month lengths, and timezone transitions.
- **Comprehensive Formatting**: Customize date-time output with flexible formatting options.
- **Conversion Utilities**: Convert between timestamps, dates, and times.

---

## Installation

To use the EasyTime library in your Rust project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
easytime = "0.1.7"
```

---

## Usage

### Create and Manipulate Time

```rust
extern crate easytime;
use easytime::EasyTime;
use chrono::Local;

fn main() {
    // Create an EasyTime instance for local time with a value of 10.
    let easy_time = EasyTime::<Local>::new(10);
    // Add and subtract time intervals
    let future_time = easy_time.seconds_from_now();
    let past_time = easy_time.minutes_ago();
    println!("Future Time (10 seconds from now): {}", future_time);
    println!("Past Time (10 minutes ago): {}", past_time);
    let past_time_local = EasyTime::<Local>::in_past(10, easy_time::TimeUnits::Days, None);
    let past_time_utc:  EasyTime::<Utc>::in_past(10, easy_time::TimeUnits::Days, None); 
    let future_time_local = EasyTime::<Local>::in_future(10, easy_time::TimeUnits::Days, None);
    let future_time_utc:  EasyTime::<Utc>::in_future(10, easy_time::TimeUnits::Days, None); 
    
}
```

### Advanced Operations

#### Working with Months and Years

```rust
use easytime::EasyTime;
use chrono::Utc;

// Create an EasyTime instance for UTC with a value of 5
let easy_time = EasyTime::<Utc>::new_with_utc(5);

// Add months and years
let future_date = easy_time.months_from_now();
let past_year = easy_time.years_ago();

println!("Future Date (5 months from now): {}", future_date);
println!("Past Date (5 years ago): {}", past_year);
```

#### Using Decades and Centuries

```rust
use easytime::EasyTime;
use chrono::Local;

let easy_time = EasyTime::<Local>::new(1); // Value represents decades or centuries

let decade_future = easy_time.decades_from_now();
let century_past = easy_time.centuries_ago();

println!("Future Date (1 decade from now): {}", decade_future);
println!("Past Date (1 century ago): {}", century_past);
```

#### Formatting and Conversion

```rust
use easytime::EasyTime;
use chrono::Local;

let easy_time = EasyTime::<Local>::new(0);

// Format current time
let formatted_time = easy_time.to_string_with_timezone();

// Convert to UNIX timestamp
let timestamp = easy_time.to_timestamp();

println!("Formatted Time: {}", formatted_time);
println!("UNIX Timestamp: {}", timestamp);
```

---

## More Examples

### Custom Time Offsets

```rust
use easytime::EasyTime;
use chrono::{Duration, Local};

let easy_time = EasyTime::<Local>::new(0);

let custom_future = easy_time.offset(Duration::days(15) + Duration::hours(10));
let custom_past = easy_time.offset_neg(Duration::minutes(90));

println!("Custom Future Time (15 days and 10 hours from now): {}", custom_future);
println!("Custom Past Time (90 minutes ago): {}", custom_past);
```

### Handling Leap Years

```rust
use easytime::EasyTime;
use chrono::{Local, NaiveDate};

let easy_time = EasyTime::<Local>::new(0);

let leap_year = EasyTime::<Local>::is_leap_year(2024);
let non_leap_year = EasyTime::<Local>::is_leap_year(2023);

println!("2024 is a leap year: {}", leap_year);
println!("2023 is not a leap year: {}", non_leap_year);
```

### Date Adjustments Across Timezones

```rust
use easytime::EasyTime;
use chrono::{Utc, Local};

let local_time = EasyTime::<Local>::new(10);
let utc_time = EasyTime::<Utc>::new_with_utc(10);

println!("Local Time 10 days from now: {}", local_time.days_from_now());
println!("UTC Time 10 days from now: {}", utc_time.days_from_now());
```

---

## API Highlights

### Constructors

- `new(value: i64)`:
  Create an instance with the current local time.
- `new_with_utc(value: i64)`:
  Create an instance with the current UTC time.

### Offset Calculations

- `seconds_from_now()`, `seconds_ago()`
- `minutes_from_now()`, `minutes_ago()`
- `hours_from_now()`, `hours_ago()`
- `days_from_now()`, `days_ago()`
- `months_from_now()`, `months_ago()`
- `years_from_now()`, `years_ago()`

### Formatting Methods

- `to_string()`: Returns a string in the format `YYYY-MM-DD HH:MM:SS`.
- `to_string_with_timezone()`: Includes timezone offset in the output.
- `to_string_with_format(format_str: &str)`: Allows custom formatting.

### Conversion Methods

- `to_timestamp()`: Converts the time to a UNIX timestamp.
- `to_date()`: Returns the date in `YYYY-MM-DD` format.
- `to_time()`: Returns the time in `HH:MM:SS` format.

---


## Rusty Rails Project

Rusty Rails is a larger project aiming to bridge the gap between Rust and Ruby/Ruby on Rails. We are actively working on recreating ruby library into rust that seamlessly make working in rust more easy and fun for new developers.

### Contributing

Contributions to the EasyTime library are welcome! Feel free to open issues, submit pull requests, or provide feedback to help improve this library.
