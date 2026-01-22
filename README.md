
# EasyTime Library

The EasyTime library is a powerful Rust crate designed to simplify date and time calculations. It builds upon the `chrono` library to provide robust support for time zone-aware operations, leap year handling, and extensive formatting options. With EasyTime, you can easily add or subtract seconds, minutes, hours, days, weeks, months, years, decades, centuries, and millennia from the current time using simple static methods.

---

## Features

- **Simple Static API**: No need to create instances - just call `EasyTime::<Local>::days_from_now(5)`.
- **Time Zone Support**: Work with local time (`Local`) or UTC time (`Utc`).
- **Flexible Date-Time Offsets**: Add or subtract time intervals with methods for seconds, minutes, hours, days, weeks, months, years, and beyond.
- **Precise Calculations**: Handles leap years, month lengths, and timezone transitions.
- **Comprehensive Formatting**: Customize date-time output with flexible formatting functions.
- **Conversion Utilities**: Convert between timestamps, dates, and times.

---

## Installation

To use the EasyTime library in your Rust project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
easy_time = "*"
```

---

## Usage

### Basic Time Calculations

```rust
use easy_time::EasyTime;
use chrono::Local;

fn main() {
    // Simple time calculations - no instance creation needed!
    let future_time = EasyTime::<Local>::seconds_from_now(10);
    let past_time = EasyTime::<Local>::minutes_ago(5);
    
    println!("10 seconds from now: {}", future_time);
    println!("5 minutes ago: {}", past_time);
    
    // Days, weeks, months, years...
    let next_week = EasyTime::<Local>::weeks_from_now(1);
    let last_month = EasyTime::<Local>::months_ago(1);
    let next_year = EasyTime::<Local>::years_from_now(1);
    
    println!("Next week: {}", next_week);
    println!("Last month: {}", last_month);
    println!("Next year: {}", next_year);
}
```

### Working with UTC

```rust
use easy_time::EasyTime;
use chrono::Utc;

fn main() {
    // UTC time calculations
    let utc_future = EasyTime::<Utc>::hours_from_now(10);
    let utc_past = EasyTime::<Utc>::days_ago(7);
    
    println!("10 hours from now (UTC): {}", utc_future);
    println!("7 days ago (UTC): {}", utc_past);
    
    // Convenience methods that return UTC time (can be called from any timezone)
    let also_utc = EasyTime::<Local>::utc_days_from_now(5);
    println!("5 days from now (UTC): {}", also_utc);
}
```

### Using Custom Base Times

```rust
use easy_time::EasyTime;
use chrono::{Local, TimeZone};

fn main() {
    // Start from a specific date instead of "now"
    let base_date = Local.with_ymd_and_hms(2024, 1, 15, 12, 0, 0).unwrap();
    
    let future = EasyTime::<Local>::days_from(base_date, 30);
    let past = EasyTime::<Local>::months_before(base_date, 3);
    
    println!("30 days after Jan 15, 2024: {}", future);
    println!("3 months before Jan 15, 2024: {}", past);
}
```

### Decades, Centuries, and Millennia

```rust
use easy_time::EasyTime;
use chrono::Local;

fn main() {
    let decade_future = EasyTime::<Local>::decades_from_now(1);
    let century_past = EasyTime::<Local>::centuries_ago(1);
    let millennium_future = EasyTime::<Local>::millenniums_from_now(1);
    
    println!("1 decade from now: {}", decade_future);
    println!("1 century ago: {}", century_past);
    println!("1 millennium from now: {}", millennium_future);
}
```

### Formatting Functions

```rust
use easy_time::{format_datetime, format_datetime_with, format_datetime_with_timezone, to_date, to_time, to_timestamp};
use chrono::Local;

fn main() {
    let now = Local::now();
    
    // Default format: YYYY-MM-DD HH:MM:SS
    println!("Default: {}", format_datetime(&now));
    
    // Custom format
    println!("Custom: {}", format_datetime_with(&now, "%Y/%m/%d"));
    
    // With timezone offset
    println!("With TZ: {}", format_datetime_with_timezone(&now));
    
    // Just date or time
    println!("Date only: {}", to_date(&now));
    println!("Time only: {}", to_time(&now));
    
    // Unix timestamp
    println!("Timestamp: {}", to_timestamp(&now));
}
```

### Handling Leap Years

```rust
use easy_time::EasyTime;
use chrono::Local;

fn main() {
    // Check if a year is a leap year
    let is_leap = EasyTime::<Local>::is_leap_year(2024);
    println!("2024 is a leap year: {}", is_leap); // true
    
    // Get days in a month (handles leap years)
    let feb_days_2024 = EasyTime::<Local>::days_in_month(2024, 2);
    let feb_days_2023 = EasyTime::<Local>::days_in_month(2023, 2);
    
    println!("Days in Feb 2024: {}", feb_days_2024); // 29
    println!("Days in Feb 2023: {}", feb_days_2023); // 28
}
```

---

## API Reference

### Time Offset Methods (Local Timezone)

All methods are static and take the offset value as a parameter:

| Method | Description |
|--------|-------------|
| `EasyTime::<Local>::seconds_from_now(n)` | n seconds in the future |
| `EasyTime::<Local>::seconds_ago(n)` | n seconds in the past |
| `EasyTime::<Local>::minutes_from_now(n)` | n minutes in the future |
| `EasyTime::<Local>::minutes_ago(n)` | n minutes in the past |
| `EasyTime::<Local>::hours_from_now(n)` | n hours in the future |
| `EasyTime::<Local>::hours_ago(n)` | n hours in the past |
| `EasyTime::<Local>::days_from_now(n)` | n days in the future |
| `EasyTime::<Local>::days_ago(n)` | n days in the past |
| `EasyTime::<Local>::weeks_from_now(n)` | n weeks in the future |
| `EasyTime::<Local>::weeks_ago(n)` | n weeks in the past |
| `EasyTime::<Local>::months_from_now(n)` | n months in the future |
| `EasyTime::<Local>::months_ago(n)` | n months in the past |
| `EasyTime::<Local>::years_from_now(n)` | n years in the future |
| `EasyTime::<Local>::years_ago(n)` | n years in the past |
| `EasyTime::<Local>::decades_from_now(n)` | n decades in the future |
| `EasyTime::<Local>::decades_ago(n)` | n decades in the past |
| `EasyTime::<Local>::centuries_from_now(n)` | n centuries in the future |
| `EasyTime::<Local>::centuries_ago(n)` | n centuries in the past |
| `EasyTime::<Local>::millenniums_from_now(n)` | n millennia in the future |
| `EasyTime::<Local>::millenniums_ago(n)` | n millennia in the past |

### Methods with Custom Base Time

For calculations from a specific date instead of "now":

| Method | Description |
|--------|-------------|
| `EasyTime::<Local>::seconds_from(base, n)` | n seconds after base |
| `EasyTime::<Local>::seconds_before(base, n)` | n seconds before base |
| `EasyTime::<Local>::days_from(base, n)` | n days after base |
| `EasyTime::<Local>::days_before(base, n)` | n days before base |
| `EasyTime::<Local>::months_from(base, n)` | n months after base |
| `EasyTime::<Local>::months_before(base, n)` | n months before base |
| ... | (all time units supported) |

### UTC Methods

Same methods are available for `EasyTime::<Utc>`:

```rust
EasyTime::<Utc>::days_from_now(5)
EasyTime::<Utc>::hours_ago(10)
```

### Convenience UTC Methods

These can be called from any timezone type and return UTC times:

```rust
EasyTime::<Local>::utc_days_from_now(5)
EasyTime::<Local>::utc_hours_ago(10)
```

### Formatting Functions

| Function | Description |
|----------|-------------|
| `format_datetime(&dt)` | Format as `YYYY-MM-DD HH:MM:SS` |
| `format_datetime_with(&dt, fmt)` | Format with custom format string |
| `format_datetime_with_timezone(&dt)` | Format with timezone offset |
| `format_datetime_with_timezone_format(&dt, fmt)` | Custom format with timezone |
| `to_date(&dt)` | Returns `YYYY-MM-DD` |
| `to_time(&dt)` | Returns `HH:MM:SS` |
| `to_timestamp(&dt)` | Returns Unix timestamp |

### Utility Methods

| Method | Description |
|--------|-------------|
| `EasyTime::<Local>::is_leap_year(year)` | Check if year is a leap year |
| `EasyTime::<Local>::days_in_month(year, month)` | Get number of days in a month |

---

## Rusty Rails Project

Rusty Rails is a larger project aiming to bridge the gap between Rust and Ruby/Ruby on Rails. We are actively working on recreating Ruby libraries in Rust to make working with Rust more easy and fun for new developers.

### Contributing

Contributions to the EasyTime library are welcome! Feel free to open issues, submit pull requests, or provide feedback to help improve this library.
