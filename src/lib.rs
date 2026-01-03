//! # Easy Time
//!
//! A simple and intuitive library for handling time in Rust.
//!
//! `easy_time` provides a convenient wrapper around [`chrono`] to make common time operations
//! more ergonomic and readable. Calculate dates in the future or past with ease using
//! human-friendly methods.
//!
//! ## Features
//!
//! - Simple API for time calculations (seconds, minutes, hours, days, months, years, etc.)
//! - Support for both local time and UTC
//! - Generic over any `chrono` timezone
//! - Handles edge cases like leap years and month boundaries
//! - Human-readable method names like `days_from_now()` and `months_ago()`
//!
//! ## Quick Start
//!
//! ```rust
//! use easy_time::{EasyTime, TimeUnits};
//! use chrono::Local;
//!
//! // Calculate 5 days from now
//! let future = EasyTime::new(5).days_from_now();
//!
//! // Calculate 3 months ago
//! let past = EasyTime::new(3).months_ago();
//!
//! // Using the convenience methods (specify Local timezone)
//! let next_week = EasyTime::<Local>::in_future(7, TimeUnits::Days, None);
//! let last_year = EasyTime::<Local>::in_past(1, TimeUnits::Years, None);
//! ```
//!
//! ## Working with UTC
//!
//! ```rust
//! use easy_time::EasyTime;
//!
//! // Create an EasyTime instance in UTC
//! let utc_time = EasyTime::new_with_utc(10);
//! let ten_hours_later = utc_time.hours_from_now();
//! ```
//!
//! ## Custom Base Time
//!
//! ```rust
//! use easy_time::EasyTime;
//! use chrono::Local;
//!
//! // Use a specific starting time instead of "now"
//! let custom_start = Local::now();
//! let easy = EasyTime::new_with_local(custom_start, 30);
//! let thirty_days_later = easy.days_from_now();
//! ```

// Allow intentional casts - year values in practice never exceed i32 range
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

use chrono::prelude::{DateTime, TimeZone};
use chrono::{Datelike, Duration, Local, LocalResult, Utc};

/// Time units for use with [`EasyTime::in_future`] and [`EasyTime::in_past`] methods.
///
/// # Example
///
/// ```rust
/// use easy_time::{EasyTime, TimeUnits};
/// use chrono::Local;
///
/// let future_date = EasyTime::<Local>::in_future(5, TimeUnits::Days, None);
/// let past_date = EasyTime::<Local>::in_past(2, TimeUnits::Months, None);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TimeUnits {
    /// Seconds
    Seconds,
    /// Minutes
    Minutes,
    /// Hours
    Hours,
    /// Days
    Days,
    /// Months (handles variable month lengths)
    Months,
    /// Years (handles leap years)
    Years,
    /// Decades (10 years)
    Decades,
    /// Centuries (100 years)
    Centuries,
    /// Millenniums (1000 years)
    Millenniums,
}

/// The main struct for time operations.
///
/// `EasyTime` wraps a `chrono::DateTime` and provides convenient methods for
/// calculating dates in the future or past. It is generic over any timezone `F`
/// that implements `chrono::TimeZone`.
///
/// # Type Parameters
///
/// * `F` - A timezone type that implements [`chrono::TimeZone`]
///
/// # Example
///
/// ```rust
/// use easy_time::EasyTime;
///
/// // Using local time
/// let easy = EasyTime::new(5);
/// let five_days_from_now = easy.days_from_now();
/// let five_days_ago = easy.days_ago();
///
/// // Using UTC
/// let easy_utc = EasyTime::new_with_utc(10);
/// let ten_hours_from_now = easy_utc.hours_from_now();
/// ```
#[derive(Clone, PartialEq, Debug, Eq)]
pub struct EasyTime<F: TimeZone> {
    /// The numeric value used for time calculations
    pub value: i64,
    /// The base datetime for calculations
    pub time_now: DateTime<F>,
}

/// Default date format: `YYYY-MM-DD HH:MM:SS`
pub const DEFAULT_DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
/// Date-only format: `YYYY-MM-DD`
pub const DATE_FORMAT: &str = "%Y-%m-%d";
/// Time-only format: `HH:MM:SS`
pub const TIME_FORMAT: &str = "%H:%M:%S";

// Constant array for days in each month (non-leap year)
const DAYS_IN_MONTH: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

/// Trait for applying time operations in forward or backward directions.
///
/// This trait provides a generic interface for time calculations that can be
/// used with any timezone.
pub trait EasyTimeOps<F: TimeZone> {
    /// Apply a time unit offset in the forward (future) direction.
    fn apply_time_unit_forward(value: i64, time_unit: TimeUnits, time: DateTime<F>) -> DateTime<F>
    where
        F::Offset: std::fmt::Display;

    /// Apply a time unit offset in the backward (past) direction.
    fn apply_time_unit_backward(value: i64, time_unit: TimeUnits, time: DateTime<F>) -> DateTime<F>
    where
        F::Offset: std::fmt::Display;
}

impl<F: TimeZone> EasyTimeOps<F> for EasyTime<F>
where
    F::Offset: std::fmt::Display,
{
    fn apply_time_unit_forward(value: i64, time_unit: TimeUnits, time: DateTime<F>) -> DateTime<F> {
        let easy_time = EasyTime::new_with_time(value, time);
        match time_unit {
            TimeUnits::Seconds => easy_time.seconds_from_now(),
            TimeUnits::Minutes => easy_time.minutes_from_now(),
            TimeUnits::Hours => easy_time.hours_from_now(),
            TimeUnits::Days => easy_time.days_from_now(),
            TimeUnits::Months => easy_time.months_from_now(),
            TimeUnits::Years => easy_time.years_from_now(),
            TimeUnits::Decades => easy_time.decades_from_now(),
            TimeUnits::Centuries => easy_time.centuries_from_now(),
            TimeUnits::Millenniums => easy_time.millenniums_from_now(),
        }
    }

    fn apply_time_unit_backward(value: i64, time_unit: TimeUnits, time: DateTime<F>) -> DateTime<F> {
        let easy_time = EasyTime::new_with_time(value, time);
        match time_unit {
            TimeUnits::Seconds => easy_time.seconds_ago(),
            TimeUnits::Minutes => easy_time.minutes_ago(),
            TimeUnits::Hours => easy_time.hours_ago(),
            TimeUnits::Days => easy_time.days_ago(),
            TimeUnits::Months => easy_time.months_ago(),
            TimeUnits::Years => easy_time.years_ago(),
            TimeUnits::Decades => easy_time.decades_ago(),
            TimeUnits::Centuries => easy_time.centuries_ago(),
            TimeUnits::Millenniums => easy_time.millenniums_ago(),
        }
    }
}

// ----------------------------------------------------------
//           EasyTime<Local>: Constructors
// ----------------------------------------------------------
impl EasyTime<Local> {
    /// Creates a new `EasyTime` with local timezone using the current time.
    ///
    /// # Arguments
    ///
    /// * `value` - The numeric value to use for time calculations
    ///
    /// # Example
    ///
    /// ```rust
    /// use easy_time::EasyTime;
    ///
    /// let easy = EasyTime::new(5);
    /// let five_days_later = easy.days_from_now();
    /// ```
    #[must_use]
    pub fn new(value: i64) -> Self {
        Self {
            value,
            time_now: Local::now(),
        }
    }

    /// Creates a new `EasyTime` with a specific local datetime as the base.
    ///
    /// # Arguments
    ///
    /// * `time` - The base datetime to use for calculations
    /// * `value` - The numeric value to use for time calculations
    ///
    /// # Example
    ///
    /// ```rust
    /// use easy_time::EasyTime;
    /// use chrono::Local;
    ///
    /// let specific_time = Local::now();
    /// let easy = EasyTime::new_with_local(specific_time, 10);
    /// ```
    #[must_use]
    pub fn new_with_local(time: DateTime<Local>, value: i64) -> Self {
        Self {
            value,
            time_now: time,
        }
    }

    /// Calculate a datetime in the future from the given (or current) time.
    ///
    /// # Arguments
    ///
    /// * `value` - The number of time units to add
    /// * `time_unit` - The unit of time (seconds, minutes, hours, etc.)
    /// * `time` - Optional base time; uses current time if `None`
    ///
    /// # Example
    ///
    /// ```rust
    /// use easy_time::{EasyTime, TimeUnits};
    /// use chrono::Local;
    ///
    /// // 5 days from now
    /// let future = EasyTime::<Local>::in_future(5, TimeUnits::Days, None);
    ///
    /// // 3 months from a specific date
    /// let base = Local::now();
    /// let future = EasyTime::<Local>::in_future(3, TimeUnits::Months, Some(base));
    /// ```
    #[must_use]
    pub fn in_future(
        value: i64,
        time_unit: TimeUnits,
        time: Option<DateTime<Local>>,
    ) -> DateTime<Local> {
        let time = time.unwrap_or_else(Local::now);
        Self::apply_time_unit_forward(value, time_unit, time)
    }

    /// Calculate a datetime in the past from the given (or current) time.
    ///
    /// # Arguments
    ///
    /// * `value` - The number of time units to subtract
    /// * `time_unit` - The unit of time (seconds, minutes, hours, etc.)
    /// * `time` - Optional base time; uses current time if `None`
    ///
    /// # Example
    ///
    /// ```rust
    /// use easy_time::{EasyTime, TimeUnits};
    /// use chrono::Local;
    ///
    /// // 2 years ago from now
    /// let past = EasyTime::<Local>::in_past(2, TimeUnits::Years, None);
    /// ```
    #[must_use]
    pub fn in_past(
        value: i64,
        time_unit: TimeUnits,
        time: Option<DateTime<Local>>,
    ) -> DateTime<Local> {
        let time = time.unwrap_or_else(Local::now);
        Self::apply_time_unit_backward(value, time_unit, time)
    }
}

// ----------------------------------------------------------
//           EasyTime<Utc>: Constructors
// ----------------------------------------------------------
impl EasyTime<Utc> {
    /// Creates a new `EasyTime` with UTC timezone using the current time.
    ///
    /// # Arguments
    ///
    /// * `value` - The numeric value to use for time calculations
    ///
    /// # Example
    ///
    /// ```rust
    /// use easy_time::EasyTime;
    ///
    /// let easy = EasyTime::new_with_utc(5);
    /// let five_hours_later = easy.hours_from_now();
    /// ```
    #[must_use]
    pub fn new_with_utc(value: i64) -> Self {
        Self {
            value,
            time_now: Utc::now(),
        }
    }

    /// Creates a new `EasyTime` with a specific UTC datetime as the base.
    ///
    /// # Arguments
    ///
    /// * `time` - The base UTC datetime to use for calculations
    /// * `value` - The numeric value to use for time calculations
    #[must_use]
    pub fn new_with_utc_time(time: DateTime<Utc>, value: i64) -> Self {
        Self {
            value,
            time_now: time,
        }
    }

    /// Calculate a UTC datetime in the future from the given (or current) time.
    ///
    /// # Arguments
    ///
    /// * `value` - The number of time units to add
    /// * `time_unit` - The unit of time (seconds, minutes, hours, etc.)
    /// * `time` - Optional base time; uses current UTC time if `None`
    #[must_use]
    pub fn in_future(
        value: i64,
        time_unit: TimeUnits,
        time: Option<DateTime<Utc>>,
    ) -> DateTime<Utc> {
        let time = time.unwrap_or_else(Utc::now);
        Self::apply_time_unit_forward(value, time_unit, time)
    }

    /// Calculate a UTC datetime in the past from the given (or current) time.
    ///
    /// # Arguments
    ///
    /// * `value` - The number of time units to subtract
    /// * `time_unit` - The unit of time (seconds, minutes, hours, etc.)
    /// * `time` - Optional base time; uses current UTC time if `None`
    #[must_use]
    pub fn in_past(value: i64, time_unit: TimeUnits, time: Option<DateTime<Utc>>) -> DateTime<Utc> {
        let time = time.unwrap_or_else(Utc::now);
        Self::apply_time_unit_backward(value, time_unit, time)
    }
}

// ----------------------------------------------------------
//   EasyTime<F> for Any TimeZone: Generic Implementation
// ----------------------------------------------------------
impl<F: TimeZone> EasyTime<F>
where
    F::Offset: std::fmt::Display,
{
    /// Creates an `EasyTime` from an existing datetime with value set to 0.
    ///
    /// Useful when you want to use formatting methods without offset calculations.
    #[must_use]
    pub fn from_time(time: DateTime<F>) -> Self {
        Self {
            value: 0,
            time_now: time,
        }
    }

    /// Creates an `EasyTime` with a specific `time_now` in any timezone `F`.
    ///
    /// # Arguments
    ///
    /// * `value` - The numeric value to use for time calculations
    /// * `time` - The base datetime in timezone `F`
    #[must_use]
    pub fn new_with_time(value: i64, time: DateTime<F>) -> Self {
        Self {
            value,
            time_now: time,
        }
    }

    // ------------------------------------------------------------------
    //                    Getters / Setters
    // ------------------------------------------------------------------

    /// Sets the value used for time calculations.
    #[inline]
    pub fn set_value(&mut self, value: i64) {
        self.value = value;
    }

    /// Sets the base datetime for calculations.
    #[inline]
    pub fn set_time(&mut self, time: DateTime<F>) {
        self.time_now = time;
    }

    /// Returns the current value.
    #[inline]
    #[must_use]
    pub fn get_value(&self) -> i64 {
        self.value
    }

    /// Returns a reference to the current base datetime.
    #[inline]
    #[must_use]
    pub fn get_time(&self) -> &DateTime<F> {
        &self.time_now
    }

    // ------------------------------------------------------------------
    //                      Internal Helpers
    // ------------------------------------------------------------------

    /// Checks if a year is a leap year.
    #[inline]
    const fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    /// Returns the number of days in a given month.
    #[inline]
    fn days_in_month(year: i32, month: u32) -> u32 {
        if month == 2 && Self::is_leap_year(year) {
            29
        } else {
            DAYS_IN_MONTH[(month - 1) as usize]
        }
    }

    /// Adds a duration to a datetime.
    #[inline]
    pub fn offset(time: &DateTime<F>, duration: Duration) -> DateTime<F> {
        time.clone() + duration
    }

    /// Subtracts a duration from a datetime.
    #[inline]
    pub fn offset_neg(time: &DateTime<F>, duration: Duration) -> DateTime<F> {
        time.clone() - duration
    }

    /// Tries to build a `DateTime<F>` from a naive date-time.
    ///
    /// Uses the first match in ambiguous cases, and panics on invalid times.
    fn build_datetime_from_naive(&self, naive: chrono::NaiveDateTime) -> DateTime<F> {
        match self.time_now.timezone().from_local_datetime(&naive) {
            LocalResult::Single(dt) => dt,
            LocalResult::Ambiguous(a, _b) => a,
            LocalResult::None => panic!("Invalid or non-existent local time."),
        }
    }

    // ------------------------------------------------------------------
    //           Simple Offsets: seconds, minutes, hours, days
    // ------------------------------------------------------------------

    /// Returns a datetime that is `value` seconds in the future.
    #[inline]
    #[must_use]
    pub fn seconds_from_now(&self) -> DateTime<F> {
        Self::offset(&self.time_now, Duration::seconds(self.value))
    }

    /// Returns a datetime that is `value` seconds in the past.
    #[inline]
    #[must_use]
    pub fn seconds_ago(&self) -> DateTime<F> {
        Self::offset_neg(&self.time_now, Duration::seconds(self.value))
    }

    /// Returns a datetime that is `value` minutes in the future.
    #[inline]
    #[must_use]
    pub fn minutes_from_now(&self) -> DateTime<F> {
        Self::offset(&self.time_now, Duration::minutes(self.value))
    }

    /// Returns a datetime that is `value` minutes in the past.
    #[inline]
    #[must_use]
    pub fn minutes_ago(&self) -> DateTime<F> {
        Self::offset_neg(&self.time_now, Duration::minutes(self.value))
    }

    /// Returns a datetime that is `value` hours in the future.
    #[inline]
    #[must_use]
    pub fn hours_from_now(&self) -> DateTime<F> {
        Self::offset(&self.time_now, Duration::hours(self.value))
    }

    /// Returns a datetime that is `value` hours in the past.
    #[inline]
    #[must_use]
    pub fn hours_ago(&self) -> DateTime<F> {
        Self::offset_neg(&self.time_now, Duration::hours(self.value))
    }

    /// Returns a datetime that is `value` days in the future.
    #[inline]
    #[must_use]
    pub fn days_from_now(&self) -> DateTime<F> {
        Self::offset(&self.time_now, Duration::days(self.value))
    }

    /// Returns a datetime that is `value` days in the past.
    #[inline]
    #[must_use]
    pub fn days_ago(&self) -> DateTime<F> {
        Self::offset_neg(&self.time_now, Duration::days(self.value))
    }

    // ------------------------------------------------------------------
    //               Month-Based Offset (custom logic)
    // ------------------------------------------------------------------

    /// Adds or subtracts months from the current datetime.
    ///
    /// Handles edge cases like different month lengths and leap years.
    /// If the target day doesn't exist in the target month, it uses the last day of that month.
    fn add_months(&self, months: i32) -> DateTime<F> {
        let naive = self.time_now.naive_local();
        let (year, month, day) = (naive.year(), naive.month() as i32, naive.day());

        // Calculate total months and use div_euclid/rem_euclid for proper negative handling
        let total_months = year * 12 + (month - 1) + months;
        let target_year = total_months.div_euclid(12);
        let target_month = total_months.rem_euclid(12) + 1;

        let days_in_target = Self::days_in_month(target_year, target_month as u32);
        let target_day = std::cmp::min(day, days_in_target);

        let target_date =
            chrono::NaiveDate::from_ymd_opt(target_year, target_month as u32, target_day)
                .expect("Invalid date after adding months");

        let target_naive_dt = target_date.and_time(naive.time());
        self.build_datetime_from_naive(target_naive_dt)
    }

    /// Returns a datetime that is `value` months in the future.
    ///
    /// Handles edge cases like months with different numbers of days.
    #[must_use]
    pub fn months_from_now(&self) -> DateTime<F> {
        self.add_months(self.value as i32)
    }

    /// Returns a datetime that is `value` months in the past.
    #[must_use]
    pub fn months_ago(&self) -> DateTime<F> {
        self.add_months(-(self.value as i32))
    }

    // ------------------------------------------------------------------
    //               Year-Based Offsets (custom logic)
    // ------------------------------------------------------------------

    /// Adds or subtracts years from the current datetime.
    ///
    /// Handles leap year edge cases (e.g., Feb 29 -> Feb 28).
    fn add_years(&self, years: i32) -> DateTime<F> {
        let naive = self.time_now.naive_local();
        let (year, month, day) = (naive.year() + years, naive.month(), naive.day());

        let days_in_target = Self::days_in_month(year, month);
        let target_day = std::cmp::min(day, days_in_target);

        let target_date = chrono::NaiveDate::from_ymd_opt(year, month, target_day)
            .expect("Invalid date after adding years");

        let target_naive_dt = target_date.and_time(naive.time());
        self.build_datetime_from_naive(target_naive_dt)
    }

    /// Returns a datetime that is `value` years in the future.
    #[must_use]
    pub fn years_from_now(&self) -> DateTime<F> {
        self.add_years(self.value as i32)
    }

    /// Returns a datetime that is `value` years in the past.
    #[must_use]
    pub fn years_ago(&self) -> DateTime<F> {
        self.add_years(-(self.value as i32))
    }

    /// Returns a datetime that is `value` decades (10 years) in the future.
    #[must_use]
    pub fn decades_from_now(&self) -> DateTime<F> {
        self.add_years(self.value as i32 * 10)
    }

    /// Returns a datetime that is `value` decades (10 years) in the past.
    #[must_use]
    pub fn decades_ago(&self) -> DateTime<F> {
        self.add_years(-(self.value as i32) * 10)
    }

    /// Returns a datetime that is `value` centuries (100 years) in the future.
    #[must_use]
    pub fn centuries_from_now(&self) -> DateTime<F> {
        self.add_years(self.value as i32 * 100)
    }

    /// Returns a datetime that is `value` centuries (100 years) in the past.
    #[must_use]
    pub fn centuries_ago(&self) -> DateTime<F> {
        self.add_years(-(self.value as i32) * 100)
    }

    /// Returns a datetime that is `value` millenniums (1000 years) in the future.
    #[must_use]
    pub fn millenniums_from_now(&self) -> DateTime<F> {
        self.add_years(self.value as i32 * 1000)
    }

    /// Returns a datetime that is `value` millenniums (1000 years) in the past.
    #[must_use]
    pub fn millenniums_ago(&self) -> DateTime<F> {
        self.add_years(-(self.value as i32) * 1000)
    }

    // ------------------------------------------------------------------
    //          Formatting Methods
    // ------------------------------------------------------------------

    /// Internal helper to format the current time with an optional timezone suffix.
    #[inline]
    fn format_with(&self, format_str: &str, show_tz: bool) -> String {
        if show_tz {
            format!("{} {}", self.time_now.format(format_str), self.time_now.offset())
        } else {
            self.time_now.format(format_str).to_string()
        }
    }

    /// Returns a string representation in the default format: `YYYY-MM-DD HH:MM:SS`.
    #[allow(clippy::inherent_to_string_shadow_display)]
    #[must_use]
    pub fn to_string(&self) -> String {
        self.format_with(DEFAULT_DATE_FORMAT, false)
    }

    /// Returns a string representation using a custom format string.
    ///
    /// See [`chrono::format::strftime`] for available format specifiers.
    #[must_use]
    pub fn to_string_with_format(&self, format_str: &str) -> String {
        self.format_with(format_str, false)
    }

    /// Returns a string representation with timezone offset appended.
    #[must_use]
    pub fn to_string_with_timezone(&self) -> String {
        self.format_with(DEFAULT_DATE_FORMAT, true)
    }

    /// Returns a string representation with custom format and timezone offset.
    #[must_use]
    pub fn to_string_with_timezone_format(&self, format_str: &str) -> String {
        self.format_with(format_str, true)
    }

    /// Alias for [`to_string_with_timezone_format`](Self::to_string_with_timezone_format).
    #[deprecated(since = "0.2.0", note = "Use to_string_with_timezone_format instead")]
    #[must_use]
    pub fn to_string_with_timezone_format_with_timezone(&self, format_str: &str) -> String {
        self.format_with(format_str, true)
    }

    // ------------------------------------------------------------------
    //           Other Utilities
    // ------------------------------------------------------------------

    /// Returns the Unix timestamp (seconds since epoch).
    #[inline]
    #[must_use]
    pub fn to_timestamp(&self) -> i64 {
        self.time_now.timestamp()
    }

    /// Returns just the date portion as `YYYY-MM-DD`.
    #[inline]
    #[must_use]
    pub fn to_date(&self) -> String {
        self.time_now.format(DATE_FORMAT).to_string()
    }

    /// Returns just the time portion as `HH:MM:SS`.
    #[inline]
    #[must_use]
    pub fn to_time(&self) -> String {
        self.time_now.format(TIME_FORMAT).to_string()
    }

    /// Returns the full datetime as `YYYY-MM-DD HH:MM:SS`.
    #[inline]
    #[must_use]
    pub fn to_date_time(&self) -> String {
        self.time_now.format(DEFAULT_DATE_FORMAT).to_string()
    }

    /// Returns datetime with custom format and timezone offset.
    #[must_use]
    pub fn to_date_time_with_timezone_format(&self, format_str: &str) -> String {
        format!("{} {}", self.time_now.format(format_str), self.time_now.offset())
    }
}

// Implementation of Display trait for better performance
impl<F: TimeZone> std::fmt::Display for EasyTime<F>
where
    F::Offset: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.time_now.format(DEFAULT_DATE_FORMAT))
    }
}
