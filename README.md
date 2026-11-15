
# EasyTime

Human-friendly date arithmetic for Rust, built on [`chrono`](https://docs.rs/chrono).

```rust
use easy_time::EasyTime;
use chrono::Local;

let next_week = EasyTime::<Local>::weeks_from_now(1);
let last_quarter = EasyTime::<Local>::months_ago(3);
```

- **Any time zone**: `Local`, `Utc`, `FixedOffset` and [`chrono-tz`](https://docs.rs/chrono-tz) zones.
- **Calendar-correct**: month ends, leap years and DST changes behave the way people expect.
- **No surprise panics**: `checked_add` returns `None` instead of overflowing, and a bad format string returns `None`.
- **Periods and words**: `start_of` and `end_of` for a day, week, month or year, and `humanize` for text like "3 days ago".

---

## Installation

```toml
[dependencies]
easy_time = "0.3"
```

`easy_time` re-exports `chrono` as `easy_time::chrono`, so you don't need a separate dependency.

---

## Usage

### Relative to now

```rust
use easy_time::EasyTime;
use chrono::{Local, Utc};

let soon = EasyTime::<Local>::minutes_from_now(15);
let yesterday = EasyTime::<Local>::days_ago(1);
let next_year_utc = EasyTime::<Utc>::years_from_now(1);
```

Every unit has `*_from_now` and `*_ago`: `seconds`, `minutes`, `hours`, `days`, `weeks`, `months`,
`years`, `decades`, `centuries`, `millenniums`.

### From a base time, in any time zone

The time zone is inferred from the base, so you don't need to name it:

```rust
use easy_time::EasyTime;
use chrono::{TimeZone, Utc};
use chrono_tz::Europe::Paris;

let base = Utc.with_ymd_and_hms(2024, 1, 31, 9, 0, 0).unwrap();
assert_eq!(EasyTime::months_from(base, 1), Utc.with_ymd_and_hms(2024, 2, 29, 9, 0, 0).unwrap());

let meeting = Paris.with_ymd_and_hms(2024, 3, 30, 10, 0, 0).unwrap();
let next = EasyTime::weeks_from(meeting, 1); // still 10:00 in Paris, after the DST change
```

Every unit has `*_from(base, n)` and `*_before(base, n)`.

### Amounts you don't control

The named methods panic if the result is out of chrono's range (about ±262,000 years).
For amounts from user input or config, use `checked_add`:

```rust
use easy_time::{EasyTime, Unit};
use chrono::Utc;

let user_supplied_days: i64 = 30;
match EasyTime::checked_add(Utc::now(), user_supplied_days, Unit::Days) {
    Some(deadline) => println!("due {deadline}"),
    None => println!("that's too far away"),
}
```

### Start and end of a period

```rust
use easy_time::{EasyTime, Unit};
use chrono::Local;

let now = Local::now();
let today_start = EasyTime::start_of(now, Unit::Days);   // 00:00:00 today
let month_end = EasyTime::end_of(now, Unit::Months);     // last nanosecond of this month
let week_start = EasyTime::start_of(now, Unit::Weeks);   // Monday 00:00:00
```

### In words

```rust
use easy_time::{humanize, humanize_delta, EasyTime};
use chrono::{TimeDelta, Utc};

assert_eq!(humanize(&EasyTime::<Utc>::days_from_now(3)), "in 3 days");
assert_eq!(humanize_delta(TimeDelta::minutes(-90)), "1 hour ago");
```

### Formatting

```rust
use easy_time::{format_datetime, format_datetime_with, format_datetime_with_timezone, to_date, to_time};
use chrono::Local;

let now = Local::now();
format_datetime(&now);                  // "2024-07-04 09:05:00"
format_datetime_with(&now, "%d/%m/%Y"); // Some("04/07/2024"), or None for an invalid format
format_datetime_with_timezone(&now);    // "2024-07-04 09:05:00 +03:00"
to_date(&now);                          // "2024-07-04"
to_time(&now);                          // "09:05:00"
```

### Calendar helpers

```rust
assert!(easy_time::is_leap_year(2024));
assert_eq!(easy_time::days_in_month(2024, 2), Some(29));
assert_eq!(easy_time::days_in_month(2024, 13), None);
```

---

## How units behave

| Units | Meaning |
|-------|---------|
| seconds, minutes, hours | Exact elapsed time. |
| days, weeks | Calendar days: the wall-clock time is kept across DST changes, so 12:00 plus one day is 12:00 the next day. |
| months, years, decades, centuries, millenniums | Calendar months: the day is clamped to the target month, so Jan 31 plus one month is Feb 28 (or 29). |

A wall-clock time that falls in a DST gap moves forward by the length of the gap (02:30 becomes
03:30). One that falls in a DST overlap keeps the base time's offset if it can, otherwise the earlier one.

Weeks start on Monday. Decades, centuries and millenniums start at years divisible by 10, 100 and 1000.

---

## Upgrading from 0.2

| 0.2 | 0.3 |
|-----|-----|
| `EasyTime::<Local>::utc_days_from_now(n)` | `EasyTime::<Utc>::days_from_now(n)` |
| `EasyTime::<Local>::is_leap_year(y)` | `easy_time::is_leap_year(y)` |
| `EasyTime::<Local>::days_in_month(y, m)` returns `u32`, panics on a bad month | `easy_time::days_in_month(y, m)` returns `Option<u32>` |
| `format_datetime_with(&dt, f)` returns `String`, panics on a bad format | returns `Option<String>` |
| `format_datetime_with_timezone_format(&dt, f)` | `format_datetime_with(&dt, &format!("{f} %:z"))` |
| `to_timestamp(&dt)` | `dt.timestamp()` |
| `*_from` / `*_before` only for `Local` and `Utc` | any `TimeZone` |
| `days_from` across a DST change moved by exactly 24 hours | keeps the wall-clock time |

---

## Rusty Rails Project

Rusty Rails is a larger project aiming to bridge the gap between Rust and Ruby/Ruby on Rails. We are actively working on recreating Ruby libraries in Rust to make working with Rust more easy and fun for new developers.

### Contributing

Contributions to the EasyTime library are welcome! Feel free to open issues, submit pull requests, or provide feedback to help improve this library.
