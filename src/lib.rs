//! # Easy Time
//!
//! A simple and intuitive library for handling time in Rust.
//!
//! `easy_time` provides a convenient wrapper around [`chrono`] to make common time operations
//! more ergonomic and readable. Calculate dates in the future or past with ease using
//! human-friendly static methods.
//!
//! ## Features
//!
//! - Simple static API for time calculations (seconds, minutes, hours, days, months, years, etc.)
//! - Support for both local time and UTC
//! - Generic over any `chrono` timezone
//! - Handles edge cases like leap years and month boundaries
//! - Human-readable method names like `days_from_now()` and `months_ago()`
//!
//! ## Quick Start
//!
//! ```rust
//! use easy_time::EasyTime;
//! use chrono::Local;
//!
//! // Calculate 5 days from now
//! let future = EasyTime::<Local>::days_from_now(5);
//!
//! // Calculate 3 months ago
//! let past = EasyTime::<Local>::months_ago(3);
//!
//! // Using UTC
//! let utc_future = EasyTime::<Local>::utc_hours_from_now(10);
//! ```

// Allow intentional casts - year values in practice never exceed i32 range
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

use chrono::prelude::{DateTime, TimeZone};
use chrono::{Datelike, Duration, Local, LocalResult, Utc};

/// Default date format: `YYYY-MM-DD HH:MM:SS`
pub const DEFAULT_DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
/// Date-only format: `YYYY-MM-DD`
pub const DATE_FORMAT: &str = "%Y-%m-%d";
/// Time-only format: `HH:MM:SS`
pub const TIME_FORMAT: &str = "%H:%M:%S";

// Constant array for days in each month (non-leap year)
const DAYS_IN_MONTH: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

/// The main struct for time operations.
///
/// `EasyTime` provides convenient static methods for calculating dates in the future or past.
/// It is generic over any timezone `F` that implements `chrono::TimeZone`.
///
/// # Type Parameters
///
/// * `F` - A timezone type that implements [`chrono::TimeZone`]
///
/// # Example
///
/// ```rust
/// use easy_time::EasyTime;
/// use chrono::{Local, Utc};
///
/// // Simple time calculations with Local timezone
/// let five_days_from_now = EasyTime::<Local>::days_from_now(5);
/// let five_days_ago = EasyTime::<Local>::days_ago(5);
///
/// // Using UTC timezone
/// let ten_hours_from_now = EasyTime::<Utc>::hours_from_now(10);
///
/// // Convenience UTC methods
/// let utc_future = EasyTime::<Local>::utc_days_from_now(7);
/// ```
#[derive(Clone, PartialEq, Debug, Eq)]
pub struct EasyTime<F: TimeZone> {
    _marker: std::marker::PhantomData<F>,
}

// ----------------------------------------------------------
//                    Internal Helpers
// ----------------------------------------------------------

/// Checks if a year is a leap year.
#[inline]
const fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Returns the number of days in a given month.
#[inline]
fn days_in_month(year: i32, month: u32) -> u32 {
    if month == 2 && is_leap_year(year) {
        29
    } else {
        DAYS_IN_MONTH[(month - 1) as usize]
    }
}

/// Tries to build a `DateTime<F>` from a naive date-time.
fn build_datetime_from_naive<F: TimeZone>(tz: &F, naive: chrono::NaiveDateTime) -> DateTime<F> {
    match tz.from_local_datetime(&naive) {
        LocalResult::Single(dt) => dt,
        LocalResult::Ambiguous(a, _b) => a,
        LocalResult::None => panic!("Invalid or non-existent local time."),
    }
}

/// Adds or subtracts months from a datetime.
fn add_months_to_datetime<F: TimeZone>(time: &DateTime<F>, months: i32) -> DateTime<F> {
    let naive = time.naive_local();
    let (year, month, day) = (naive.year(), naive.month() as i32, naive.day());

    let total_months = year * 12 + (month - 1) + months;
    let target_year = total_months.div_euclid(12);
    let target_month = total_months.rem_euclid(12) + 1;

    let days_in_target = days_in_month(target_year, target_month as u32);
    let target_day = std::cmp::min(day, days_in_target);

    let target_date =
        chrono::NaiveDate::from_ymd_opt(target_year, target_month as u32, target_day)
            .expect("Invalid date after adding months");

    let target_naive_dt = target_date.and_time(naive.time());
    build_datetime_from_naive(&time.timezone(), target_naive_dt)
}

/// Adds or subtracts years from a datetime.
fn add_years_to_datetime<F: TimeZone>(time: &DateTime<F>, years: i32) -> DateTime<F> {
    let naive = time.naive_local();
    let (year, month, day) = (naive.year() + years, naive.month(), naive.day());

    let days_in_target = days_in_month(year, month);
    let target_day = std::cmp::min(day, days_in_target);

    let target_date = chrono::NaiveDate::from_ymd_opt(year, month, target_day)
        .expect("Invalid date after adding years");

    let target_naive_dt = target_date.and_time(naive.time());
    build_datetime_from_naive(&time.timezone(), target_naive_dt)
}

// ----------------------------------------------------------
//           EasyTime<Local>: Static Methods
// ----------------------------------------------------------
impl EasyTime<Local> {
    // ------------------------------------------------------------------
    //           Simple Offsets: seconds, minutes, hours, days
    // ------------------------------------------------------------------

    /// Returns a datetime that is `value` seconds in the future from now.
    ///
    /// # Example
    /// ```rust
    /// use easy_time::EasyTime;
    /// use chrono::Local;
    ///
    /// let future = EasyTime::<Local>::seconds_from_now(30);
    /// ```
    #[inline]
    #[must_use]
    pub fn seconds_from_now(value: i64) -> DateTime<Local> {
        Local::now() + Duration::seconds(value)
    }

    /// Returns a datetime that is `value` seconds in the past from now.
    #[inline]
    #[must_use]
    pub fn seconds_ago(value: i64) -> DateTime<Local> {
        Local::now() - Duration::seconds(value)
    }

    /// Returns a datetime that is `value` minutes in the future from now.
    #[inline]
    #[must_use]
    pub fn minutes_from_now(value: i64) -> DateTime<Local> {
        Local::now() + Duration::minutes(value)
    }

    /// Returns a datetime that is `value` minutes in the past from now.
    #[inline]
    #[must_use]
    pub fn minutes_ago(value: i64) -> DateTime<Local> {
        Local::now() - Duration::minutes(value)
    }

    /// Returns a datetime that is `value` hours in the future from now.
    #[inline]
    #[must_use]
    pub fn hours_from_now(value: i64) -> DateTime<Local> {
        Local::now() + Duration::hours(value)
    }

    /// Returns a datetime that is `value` hours in the past from now.
    #[inline]
    #[must_use]
    pub fn hours_ago(value: i64) -> DateTime<Local> {
        Local::now() - Duration::hours(value)
    }

    /// Returns a datetime that is `value` days in the future from now.
    #[inline]
    #[must_use]
    pub fn days_from_now(value: i64) -> DateTime<Local> {
        Local::now() + Duration::days(value)
    }

    /// Returns a datetime that is `value` days in the past from now.
    #[inline]
    #[must_use]
    pub fn days_ago(value: i64) -> DateTime<Local> {
        Local::now() - Duration::days(value)
    }

    /// Returns a datetime that is `value` weeks in the future from now.
    #[inline]
    #[must_use]
    pub fn weeks_from_now(value: i64) -> DateTime<Local> {
        Local::now() + Duration::weeks(value)
    }

    /// Returns a datetime that is `value` weeks in the past from now.
    #[inline]
    #[must_use]
    pub fn weeks_ago(value: i64) -> DateTime<Local> {
        Local::now() - Duration::weeks(value)
    }

    // ------------------------------------------------------------------
    //               Month-Based Offsets
    // ------------------------------------------------------------------

    /// Returns a datetime that is `value` months in the future from now.
    ///
    /// Handles edge cases like months with different numbers of days.
    #[must_use]
    pub fn months_from_now(value: i64) -> DateTime<Local> {
        add_months_to_datetime(&Local::now(), value as i32)
    }

    /// Returns a datetime that is `value` months in the past from now.
    #[must_use]
    pub fn months_ago(value: i64) -> DateTime<Local> {
        add_months_to_datetime(&Local::now(), -(value as i32))
    }

    // ------------------------------------------------------------------
    //               Year-Based Offsets
    // ------------------------------------------------------------------

    /// Returns a datetime that is `value` years in the future from now.
    #[must_use]
    pub fn years_from_now(value: i64) -> DateTime<Local> {
        add_years_to_datetime(&Local::now(), value as i32)
    }

    /// Returns a datetime that is `value` years in the past from now.
    #[must_use]
    pub fn years_ago(value: i64) -> DateTime<Local> {
        add_years_to_datetime(&Local::now(), -(value as i32))
    }

    /// Returns a datetime that is `value` decades (10 years) in the future from now.
    #[must_use]
    pub fn decades_from_now(value: i64) -> DateTime<Local> {
        add_years_to_datetime(&Local::now(), value as i32 * 10)
    }

    /// Returns a datetime that is `value` decades (10 years) in the past from now.
    #[must_use]
    pub fn decades_ago(value: i64) -> DateTime<Local> {
        add_years_to_datetime(&Local::now(), -(value as i32) * 10)
    }

    /// Returns a datetime that is `value` centuries (100 years) in the future from now.
    #[must_use]
    pub fn centuries_from_now(value: i64) -> DateTime<Local> {
        add_years_to_datetime(&Local::now(), value as i32 * 100)
    }

    /// Returns a datetime that is `value` centuries (100 years) in the past from now.
    #[must_use]
    pub fn centuries_ago(value: i64) -> DateTime<Local> {
        add_years_to_datetime(&Local::now(), -(value as i32) * 100)
    }

    /// Returns a datetime that is `value` millenniums (1000 years) in the future from now.
    #[must_use]
    pub fn millenniums_from_now(value: i64) -> DateTime<Local> {
        add_years_to_datetime(&Local::now(), value as i32 * 1000)
    }

    /// Returns a datetime that is `value` millenniums (1000 years) in the past from now.
    #[must_use]
    pub fn millenniums_ago(value: i64) -> DateTime<Local> {
        add_years_to_datetime(&Local::now(), -(value as i32) * 1000)
    }

    // ------------------------------------------------------------------
    //           Methods with custom base time
    // ------------------------------------------------------------------

    /// Returns a datetime that is `value` seconds from the given base time.
    #[inline]
    #[must_use]
    pub fn seconds_from(base: DateTime<Local>, value: i64) -> DateTime<Local> {
        base + Duration::seconds(value)
    }

    /// Returns a datetime that is `value` seconds before the given base time.
    #[inline]
    #[must_use]
    pub fn seconds_before(base: DateTime<Local>, value: i64) -> DateTime<Local> {
        base - Duration::seconds(value)
    }

    /// Returns a datetime that is `value` minutes from the given base time.
    #[inline]
    #[must_use]
    pub fn minutes_from(base: DateTime<Local>, value: i64) -> DateTime<Local> {
        base + Duration::minutes(value)
    }

    /// Returns a datetime that is `value` minutes before the given base time.
    #[inline]
    #[must_use]
    pub fn minutes_before(base: DateTime<Local>, value: i64) -> DateTime<Local> {
        base - Duration::minutes(value)
    }

    /// Returns a datetime that is `value` hours from the given base time.
    #[inline]
    #[must_use]
    pub fn hours_from(base: DateTime<Local>, value: i64) -> DateTime<Local> {
        base + Duration::hours(value)
    }

    /// Returns a datetime that is `value` hours before the given base time.
    #[inline]
    #[must_use]
    pub fn hours_before(base: DateTime<Local>, value: i64) -> DateTime<Local> {
        base - Duration::hours(value)
    }

    /// Returns a datetime that is `value` days from the given base time.
    #[inline]
    #[must_use]
    pub fn days_from(base: DateTime<Local>, value: i64) -> DateTime<Local> {
        base + Duration::days(value)
    }

    /// Returns a datetime that is `value` days before the given base time.
    #[inline]
    #[must_use]
    pub fn days_before(base: DateTime<Local>, value: i64) -> DateTime<Local> {
        base - Duration::days(value)
    }

    /// Returns a datetime that is `value` weeks from the given base time.
    #[inline]
    #[must_use]
    pub fn weeks_from(base: DateTime<Local>, value: i64) -> DateTime<Local> {
        base + Duration::weeks(value)
    }

    /// Returns a datetime that is `value` weeks before the given base time.
    #[inline]
    #[must_use]
    pub fn weeks_before(base: DateTime<Local>, value: i64) -> DateTime<Local> {
        base - Duration::weeks(value)
    }

    /// Returns a datetime that is `value` months from the given base time.
    #[must_use]
    pub fn months_from(base: DateTime<Local>, value: i64) -> DateTime<Local> {
        add_months_to_datetime(&base, value as i32)
    }

    /// Returns a datetime that is `value` months before the given base time.
    #[must_use]
    pub fn months_before(base: DateTime<Local>, value: i64) -> DateTime<Local> {
        add_months_to_datetime(&base, -(value as i32))
    }

    /// Returns a datetime that is `value` years from the given base time.
    #[must_use]
    pub fn years_from(base: DateTime<Local>, value: i64) -> DateTime<Local> {
        add_years_to_datetime(&base, value as i32)
    }

    /// Returns a datetime that is `value` years before the given base time.
    #[must_use]
    pub fn years_before(base: DateTime<Local>, value: i64) -> DateTime<Local> {
        add_years_to_datetime(&base, -(value as i32))
    }

    /// Returns a datetime that is `value` decades from the given base time.
    #[must_use]
    pub fn decades_from(base: DateTime<Local>, value: i64) -> DateTime<Local> {
        add_years_to_datetime(&base, value as i32 * 10)
    }

    /// Returns a datetime that is `value` decades before the given base time.
    #[must_use]
    pub fn decades_before(base: DateTime<Local>, value: i64) -> DateTime<Local> {
        add_years_to_datetime(&base, -(value as i32) * 10)
    }

    /// Returns a datetime that is `value` centuries from the given base time.
    #[must_use]
    pub fn centuries_from(base: DateTime<Local>, value: i64) -> DateTime<Local> {
        add_years_to_datetime(&base, value as i32 * 100)
    }

    /// Returns a datetime that is `value` centuries before the given base time.
    #[must_use]
    pub fn centuries_before(base: DateTime<Local>, value: i64) -> DateTime<Local> {
        add_years_to_datetime(&base, -(value as i32) * 100)
    }

    /// Returns a datetime that is `value` millenniums from the given base time.
    #[must_use]
    pub fn millenniums_from(base: DateTime<Local>, value: i64) -> DateTime<Local> {
        add_years_to_datetime(&base, value as i32 * 1000)
    }

    /// Returns a datetime that is `value` millenniums before the given base time.
    #[must_use]
    pub fn millenniums_before(base: DateTime<Local>, value: i64) -> DateTime<Local> {
        add_years_to_datetime(&base, -(value as i32) * 1000)
    }
}

// ----------------------------------------------------------
//           EasyTime<Utc>: Static Methods
// ----------------------------------------------------------
impl EasyTime<Utc> {
    // ------------------------------------------------------------------
    //           Simple Offsets: seconds, minutes, hours, days
    // ------------------------------------------------------------------

    /// Returns a UTC datetime that is `value` seconds in the future from now.
    #[inline]
    #[must_use]
    pub fn seconds_from_now(value: i64) -> DateTime<Utc> {
        Utc::now() + Duration::seconds(value)
    }

    /// Returns a UTC datetime that is `value` seconds in the past from now.
    #[inline]
    #[must_use]
    pub fn seconds_ago(value: i64) -> DateTime<Utc> {
        Utc::now() - Duration::seconds(value)
    }

    /// Returns a UTC datetime that is `value` minutes in the future from now.
    #[inline]
    #[must_use]
    pub fn minutes_from_now(value: i64) -> DateTime<Utc> {
        Utc::now() + Duration::minutes(value)
    }

    /// Returns a UTC datetime that is `value` minutes in the past from now.
    #[inline]
    #[must_use]
    pub fn minutes_ago(value: i64) -> DateTime<Utc> {
        Utc::now() - Duration::minutes(value)
    }

    /// Returns a UTC datetime that is `value` hours in the future from now.
    #[inline]
    #[must_use]
    pub fn hours_from_now(value: i64) -> DateTime<Utc> {
        Utc::now() + Duration::hours(value)
    }

    /// Returns a UTC datetime that is `value` hours in the past from now.
    #[inline]
    #[must_use]
    pub fn hours_ago(value: i64) -> DateTime<Utc> {
        Utc::now() - Duration::hours(value)
    }

    /// Returns a UTC datetime that is `value` days in the future from now.
    #[inline]
    #[must_use]
    pub fn days_from_now(value: i64) -> DateTime<Utc> {
        Utc::now() + Duration::days(value)
    }

    /// Returns a UTC datetime that is `value` days in the past from now.
    #[inline]
    #[must_use]
    pub fn days_ago(value: i64) -> DateTime<Utc> {
        Utc::now() - Duration::days(value)
    }

    /// Returns a UTC datetime that is `value` weeks in the future from now.
    #[inline]
    #[must_use]
    pub fn weeks_from_now(value: i64) -> DateTime<Utc> {
        Utc::now() + Duration::weeks(value)
    }

    /// Returns a UTC datetime that is `value` weeks in the past from now.
    #[inline]
    #[must_use]
    pub fn weeks_ago(value: i64) -> DateTime<Utc> {
        Utc::now() - Duration::weeks(value)
    }

    // ------------------------------------------------------------------
    //               Month-Based Offsets
    // ------------------------------------------------------------------

    /// Returns a UTC datetime that is `value` months in the future from now.
    #[must_use]
    pub fn months_from_now(value: i64) -> DateTime<Utc> {
        add_months_to_datetime(&Utc::now(), value as i32)
    }

    /// Returns a UTC datetime that is `value` months in the past from now.
    #[must_use]
    pub fn months_ago(value: i64) -> DateTime<Utc> {
        add_months_to_datetime(&Utc::now(), -(value as i32))
    }

    // ------------------------------------------------------------------
    //               Year-Based Offsets
    // ------------------------------------------------------------------

    /// Returns a UTC datetime that is `value` years in the future from now.
    #[must_use]
    pub fn years_from_now(value: i64) -> DateTime<Utc> {
        add_years_to_datetime(&Utc::now(), value as i32)
    }

    /// Returns a UTC datetime that is `value` years in the past from now.
    #[must_use]
    pub fn years_ago(value: i64) -> DateTime<Utc> {
        add_years_to_datetime(&Utc::now(), -(value as i32))
    }

    /// Returns a UTC datetime that is `value` decades (10 years) in the future from now.
    #[must_use]
    pub fn decades_from_now(value: i64) -> DateTime<Utc> {
        add_years_to_datetime(&Utc::now(), value as i32 * 10)
    }

    /// Returns a UTC datetime that is `value` decades (10 years) in the past from now.
    #[must_use]
    pub fn decades_ago(value: i64) -> DateTime<Utc> {
        add_years_to_datetime(&Utc::now(), -(value as i32) * 10)
    }

    /// Returns a UTC datetime that is `value` centuries (100 years) in the future from now.
    #[must_use]
    pub fn centuries_from_now(value: i64) -> DateTime<Utc> {
        add_years_to_datetime(&Utc::now(), value as i32 * 100)
    }

    /// Returns a UTC datetime that is `value` centuries (100 years) in the past from now.
    #[must_use]
    pub fn centuries_ago(value: i64) -> DateTime<Utc> {
        add_years_to_datetime(&Utc::now(), -(value as i32) * 100)
    }

    /// Returns a UTC datetime that is `value` millenniums (1000 years) in the future from now.
    #[must_use]
    pub fn millenniums_from_now(value: i64) -> DateTime<Utc> {
        add_years_to_datetime(&Utc::now(), value as i32 * 1000)
    }

    /// Returns a UTC datetime that is `value` millenniums (1000 years) in the past from now.
    #[must_use]
    pub fn millenniums_ago(value: i64) -> DateTime<Utc> {
        add_years_to_datetime(&Utc::now(), -(value as i32) * 1000)
    }

    // ------------------------------------------------------------------
    //           Methods with custom base time
    // ------------------------------------------------------------------

    /// Returns a UTC datetime that is `value` seconds from the given base time.
    #[inline]
    #[must_use]
    pub fn seconds_from(base: DateTime<Utc>, value: i64) -> DateTime<Utc> {
        base + Duration::seconds(value)
    }

    /// Returns a UTC datetime that is `value` seconds before the given base time.
    #[inline]
    #[must_use]
    pub fn seconds_before(base: DateTime<Utc>, value: i64) -> DateTime<Utc> {
        base - Duration::seconds(value)
    }

    /// Returns a UTC datetime that is `value` minutes from the given base time.
    #[inline]
    #[must_use]
    pub fn minutes_from(base: DateTime<Utc>, value: i64) -> DateTime<Utc> {
        base + Duration::minutes(value)
    }

    /// Returns a UTC datetime that is `value` minutes before the given base time.
    #[inline]
    #[must_use]
    pub fn minutes_before(base: DateTime<Utc>, value: i64) -> DateTime<Utc> {
        base - Duration::minutes(value)
    }

    /// Returns a UTC datetime that is `value` hours from the given base time.
    #[inline]
    #[must_use]
    pub fn hours_from(base: DateTime<Utc>, value: i64) -> DateTime<Utc> {
        base + Duration::hours(value)
    }

    /// Returns a UTC datetime that is `value` hours before the given base time.
    #[inline]
    #[must_use]
    pub fn hours_before(base: DateTime<Utc>, value: i64) -> DateTime<Utc> {
        base - Duration::hours(value)
    }

    /// Returns a UTC datetime that is `value` days from the given base time.
    #[inline]
    #[must_use]
    pub fn days_from(base: DateTime<Utc>, value: i64) -> DateTime<Utc> {
        base + Duration::days(value)
    }

    /// Returns a UTC datetime that is `value` days before the given base time.
    #[inline]
    #[must_use]
    pub fn days_before(base: DateTime<Utc>, value: i64) -> DateTime<Utc> {
        base - Duration::days(value)
    }

    /// Returns a UTC datetime that is `value` weeks from the given base time.
    #[inline]
    #[must_use]
    pub fn weeks_from(base: DateTime<Utc>, value: i64) -> DateTime<Utc> {
        base + Duration::weeks(value)
    }

    /// Returns a UTC datetime that is `value` weeks before the given base time.
    #[inline]
    #[must_use]
    pub fn weeks_before(base: DateTime<Utc>, value: i64) -> DateTime<Utc> {
        base - Duration::weeks(value)
    }

    /// Returns a UTC datetime that is `value` months from the given base time.
    #[must_use]
    pub fn months_from(base: DateTime<Utc>, value: i64) -> DateTime<Utc> {
        add_months_to_datetime(&base, value as i32)
    }

    /// Returns a UTC datetime that is `value` months before the given base time.
    #[must_use]
    pub fn months_before(base: DateTime<Utc>, value: i64) -> DateTime<Utc> {
        add_months_to_datetime(&base, -(value as i32))
    }

    /// Returns a UTC datetime that is `value` years from the given base time.
    #[must_use]
    pub fn years_from(base: DateTime<Utc>, value: i64) -> DateTime<Utc> {
        add_years_to_datetime(&base, value as i32)
    }

    /// Returns a UTC datetime that is `value` years before the given base time.
    #[must_use]
    pub fn years_before(base: DateTime<Utc>, value: i64) -> DateTime<Utc> {
        add_years_to_datetime(&base, -(value as i32))
    }

    /// Returns a UTC datetime that is `value` decades from the given base time.
    #[must_use]
    pub fn decades_from(base: DateTime<Utc>, value: i64) -> DateTime<Utc> {
        add_years_to_datetime(&base, value as i32 * 10)
    }

    /// Returns a UTC datetime that is `value` decades before the given base time.
    #[must_use]
    pub fn decades_before(base: DateTime<Utc>, value: i64) -> DateTime<Utc> {
        add_years_to_datetime(&base, -(value as i32) * 10)
    }

    /// Returns a UTC datetime that is `value` centuries from the given base time.
    #[must_use]
    pub fn centuries_from(base: DateTime<Utc>, value: i64) -> DateTime<Utc> {
        add_years_to_datetime(&base, value as i32 * 100)
    }

    /// Returns a UTC datetime that is `value` centuries before the given base time.
    #[must_use]
    pub fn centuries_before(base: DateTime<Utc>, value: i64) -> DateTime<Utc> {
        add_years_to_datetime(&base, -(value as i32) * 100)
    }

    /// Returns a UTC datetime that is `value` millenniums from the given base time.
    #[must_use]
    pub fn millenniums_from(base: DateTime<Utc>, value: i64) -> DateTime<Utc> {
        add_years_to_datetime(&base, value as i32 * 1000)
    }

    /// Returns a UTC datetime that is `value` millenniums before the given base time.
    #[must_use]
    pub fn millenniums_before(base: DateTime<Utc>, value: i64) -> DateTime<Utc> {
        add_years_to_datetime(&base, -(value as i32) * 1000)
    }
}

// ----------------------------------------------------------
//           Convenience UTC methods (no type parameter needed)
// ----------------------------------------------------------
impl<F: TimeZone> EasyTime<F> {
    /// Returns a UTC datetime that is `value` seconds in the future from now.
    ///
    /// Convenience method that doesn't require specifying a type parameter.
    ///
    /// # Example
    /// ```rust
    /// use easy_time::EasyTime;
    /// use chrono::Local;
    ///
    /// let future = EasyTime::<Local>::utc_seconds_from_now(30);
    /// ```
    #[inline]
    #[must_use]
    pub fn utc_seconds_from_now(value: i64) -> DateTime<Utc> {
        Utc::now() + Duration::seconds(value)
    }

    /// Returns a UTC datetime that is `value` seconds in the past from now.
    #[inline]
    #[must_use]
    pub fn utc_seconds_ago(value: i64) -> DateTime<Utc> {
        Utc::now() - Duration::seconds(value)
    }

    /// Returns a UTC datetime that is `value` minutes in the future from now.
    #[inline]
    #[must_use]
    pub fn utc_minutes_from_now(value: i64) -> DateTime<Utc> {
        Utc::now() + Duration::minutes(value)
    }

    /// Returns a UTC datetime that is `value` minutes in the past from now.
    #[inline]
    #[must_use]
    pub fn utc_minutes_ago(value: i64) -> DateTime<Utc> {
        Utc::now() - Duration::minutes(value)
    }

    /// Returns a UTC datetime that is `value` hours in the future from now.
    #[inline]
    #[must_use]
    pub fn utc_hours_from_now(value: i64) -> DateTime<Utc> {
        Utc::now() + Duration::hours(value)
    }

    /// Returns a UTC datetime that is `value` hours in the past from now.
    #[inline]
    #[must_use]
    pub fn utc_hours_ago(value: i64) -> DateTime<Utc> {
        Utc::now() - Duration::hours(value)
    }

    /// Returns a UTC datetime that is `value` days in the future from now.
    #[inline]
    #[must_use]
    pub fn utc_days_from_now(value: i64) -> DateTime<Utc> {
        Utc::now() + Duration::days(value)
    }

    /// Returns a UTC datetime that is `value` days in the past from now.
    #[inline]
    #[must_use]
    pub fn utc_days_ago(value: i64) -> DateTime<Utc> {
        Utc::now() - Duration::days(value)
    }

    /// Returns a UTC datetime that is `value` weeks in the future from now.
    #[inline]
    #[must_use]
    pub fn utc_weeks_from_now(value: i64) -> DateTime<Utc> {
        Utc::now() + Duration::weeks(value)
    }

    /// Returns a UTC datetime that is `value` weeks in the past from now.
    #[inline]
    #[must_use]
    pub fn utc_weeks_ago(value: i64) -> DateTime<Utc> {
        Utc::now() - Duration::weeks(value)
    }

    /// Returns a UTC datetime that is `value` months in the future from now.
    #[must_use]
    pub fn utc_months_from_now(value: i64) -> DateTime<Utc> {
        add_months_to_datetime(&Utc::now(), value as i32)
    }

    /// Returns a UTC datetime that is `value` months in the past from now.
    #[must_use]
    pub fn utc_months_ago(value: i64) -> DateTime<Utc> {
        add_months_to_datetime(&Utc::now(), -(value as i32))
    }

    /// Returns a UTC datetime that is `value` years in the future from now.
    #[must_use]
    pub fn utc_years_from_now(value: i64) -> DateTime<Utc> {
        add_years_to_datetime(&Utc::now(), value as i32)
    }

    /// Returns a UTC datetime that is `value` years in the past from now.
    #[must_use]
    pub fn utc_years_ago(value: i64) -> DateTime<Utc> {
        add_years_to_datetime(&Utc::now(), -(value as i32))
    }

    /// Returns a UTC datetime that is `value` decades in the future from now.
    #[must_use]
    pub fn utc_decades_from_now(value: i64) -> DateTime<Utc> {
        add_years_to_datetime(&Utc::now(), value as i32 * 10)
    }

    /// Returns a UTC datetime that is `value` decades in the past from now.
    #[must_use]
    pub fn utc_decades_ago(value: i64) -> DateTime<Utc> {
        add_years_to_datetime(&Utc::now(), -(value as i32) * 10)
    }

    /// Returns a UTC datetime that is `value` centuries in the future from now.
    #[must_use]
    pub fn utc_centuries_from_now(value: i64) -> DateTime<Utc> {
        add_years_to_datetime(&Utc::now(), value as i32 * 100)
    }

    /// Returns a UTC datetime that is `value` centuries in the past from now.
    #[must_use]
    pub fn utc_centuries_ago(value: i64) -> DateTime<Utc> {
        add_years_to_datetime(&Utc::now(), -(value as i32) * 100)
    }

    /// Returns a UTC datetime that is `value` millenniums in the future from now.
    #[must_use]
    pub fn utc_millenniums_from_now(value: i64) -> DateTime<Utc> {
        add_years_to_datetime(&Utc::now(), value as i32 * 1000)
    }

    /// Returns a UTC datetime that is `value` millenniums in the past from now.
    #[must_use]
    pub fn utc_millenniums_ago(value: i64) -> DateTime<Utc> {
        add_years_to_datetime(&Utc::now(), -(value as i32) * 1000)
    }

    // ------------------------------------------------------------------
    //           Utility Functions
    // ------------------------------------------------------------------

    /// Checks if a year is a leap year.
    ///
    /// # Example
    /// ```rust
    /// use easy_time::EasyTime;
    /// use chrono::Local;
    ///
    /// assert!(EasyTime::<Local>::is_leap_year(2024));
    /// assert!(!EasyTime::<Local>::is_leap_year(2023));
    /// ```
    #[inline]
    #[must_use]
    pub fn is_leap_year(year: i32) -> bool {
        is_leap_year(year)
    }

    /// Returns the number of days in a given month.
    ///
    /// # Example
    /// ```rust
    /// use easy_time::EasyTime;
    /// use chrono::Local;
    ///
    /// assert_eq!(EasyTime::<Local>::days_in_month(2024, 2), 29); // Leap year
    /// assert_eq!(EasyTime::<Local>::days_in_month(2023, 2), 28); // Non-leap year
    /// ```
    #[inline]
    #[must_use]
    pub fn days_in_month(year: i32, month: u32) -> u32 {
        days_in_month(year, month)
    }
}

// ----------------------------------------------------------
//           Formatting Utilities (standalone functions)
// ----------------------------------------------------------

/// Formats a datetime using the default format: `YYYY-MM-DD HH:MM:SS`.
#[must_use]
pub fn format_datetime<F: TimeZone>(time: &DateTime<F>) -> String
where
    F::Offset: std::fmt::Display,
{
    time.format(DEFAULT_DATE_FORMAT).to_string()
}

/// Formats a datetime using a custom format string.
#[must_use]
pub fn format_datetime_with<F: TimeZone>(time: &DateTime<F>, format_str: &str) -> String
where
    F::Offset: std::fmt::Display,
{
    time.format(format_str).to_string()
}

/// Formats a datetime with timezone offset appended.
#[must_use]
pub fn format_datetime_with_timezone<F: TimeZone>(time: &DateTime<F>) -> String
where
    F::Offset: std::fmt::Display,
{
    format!("{} {}", time.format(DEFAULT_DATE_FORMAT), time.offset())
}

/// Formats a datetime with custom format and timezone offset.
#[must_use]
pub fn format_datetime_with_timezone_format<F: TimeZone>(
    time: &DateTime<F>,
    format_str: &str,
) -> String
where
    F::Offset: std::fmt::Display,
{
    format!("{} {}", time.format(format_str), time.offset())
}

/// Returns just the date portion as `YYYY-MM-DD`.
#[must_use]
pub fn to_date<F: TimeZone>(time: &DateTime<F>) -> String
where
    F::Offset: std::fmt::Display,
{
    time.format(DATE_FORMAT).to_string()
}

/// Returns just the time portion as `HH:MM:SS`.
#[must_use]
pub fn to_time<F: TimeZone>(time: &DateTime<F>) -> String
where
    F::Offset: std::fmt::Display,
{
    time.format(TIME_FORMAT).to_string()
}

/// Returns the Unix timestamp (seconds since epoch).
#[must_use]
pub fn to_timestamp<F: TimeZone>(time: &DateTime<F>) -> i64 {
    time.timestamp()
}
