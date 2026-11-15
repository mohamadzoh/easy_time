//! # Easy Time
//!
//! Human-friendly date arithmetic on top of [`chrono`].
//!
//! ```rust
//! use easy_time::{EasyTime, Unit};
//! use chrono::{Local, TimeZone, Utc};
//!
//! // Relative to now, in Local or Utc:
//! let next_week = EasyTime::<Local>::weeks_from_now(1);
//! let last_quarter = EasyTime::<Utc>::months_ago(3);
//!
//! // From any base time, in any time zone (the zone is inferred from the base):
//! let base = Utc.with_ymd_and_hms(2024, 1, 31, 9, 0, 0).unwrap();
//! assert_eq!(EasyTime::months_from(base, 1), Utc.with_ymd_and_hms(2024, 2, 29, 9, 0, 0).unwrap());
//!
//! // Start and end of a period:
//! assert_eq!(EasyTime::start_of(base, Unit::Months), Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());
//!
//! // In words:
//! assert_eq!(easy_time::humanize_delta(chrono::TimeDelta::hours(-3)), "3 hours ago");
//! ```
//!
//! ## How units behave
//!
//! - Seconds, minutes and hours are exact elapsed time.
//! - Days and longer are calendar units that keep the wall-clock time, even across a DST change:
//!   12:00 plus one day is 12:00 the next day, though 23 or 25 hours may have passed.
//! - Months and longer clamp the day to the target month: Jan 31 plus one month is Feb 28 (or 29).
//! - A wall-clock time that falls in a DST gap moves forward by the length of the gap (02:30 becomes
//!   03:30). One that falls in a DST overlap keeps the base time's offset if it can, else the earlier one.
//!
//! The named methods (`days_from`, `months_ago`, ...) panic if the result is outside chrono's range
//! (about ±262,000 years). Use [`EasyTime::checked_add`] for amounts you don't control.

#![warn(missing_docs)]

pub use chrono;

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;

use chrono::{
    DateTime, Datelike, Days, Local, MappedLocalTime, Months, NaiveDate, NaiveDateTime, Offset,
    TimeDelta, TimeZone, Timelike, Utc,
};
use std::fmt::{Display, Write};
use std::marker::PhantomData;

/// Default date format: `YYYY-MM-DD HH:MM:SS`
pub const DEFAULT_DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
/// Date-only format: `YYYY-MM-DD`
pub const DATE_FORMAT: &str = "%Y-%m-%d";
/// Time-only format: `HH:MM:SS`
pub const TIME_FORMAT: &str = "%H:%M:%S";

const OUT_OF_RANGE: &str = "easy_time: date out of range";

/// A unit of time for [`EasyTime::checked_add`], [`EasyTime::start_of`] and [`EasyTime::end_of`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Unit {
    /// One second.
    Seconds,
    /// 60 seconds.
    Minutes,
    /// 60 minutes.
    Hours,
    /// A calendar day.
    Days,
    /// Seven calendar days. Weeks start on Monday.
    Weeks,
    /// A calendar month.
    Months,
    /// 12 months.
    Years,
    /// 10 years. Decades start at years divisible by 10.
    Decades,
    /// 100 years. Centuries start at years divisible by 100.
    Centuries,
    /// 1000 years. Millenniums start at years divisible by 1000.
    Millenniums,
}

/// Time zones that can tell the current time on their own: [`Local`] and [`Utc`].
///
/// Implement it for your own zero-sized zone to get the `*_from_now` and `*_ago` methods.
pub trait Now: TimeZone {
    /// The current time in this zone.
    fn now() -> DateTime<Self>;
}

impl Now for Local {
    fn now() -> DateTime<Self> {
        Local::now()
    }
}

impl Now for Utc {
    fn now() -> DateTime<Self> {
        Utc::now()
    }
}

/// Date arithmetic in the time zone `Tz`.
///
/// `EasyTime` is never constructed; every function is called on the type, like
/// `EasyTime::<Local>::days_from_now(5)` or `EasyTime::days_from(base, 5)`.
/// See the [crate docs](crate) for how each unit behaves across DST and month ends.
///
/// ```rust
/// use easy_time::EasyTime;
/// use chrono::{FixedOffset, Local, TimeZone, Utc};
///
/// let soon = EasyTime::<Local>::hours_from_now(2);
/// let then = EasyTime::<Utc>::years_ago(1);
///
/// let beirut = FixedOffset::east_opt(3 * 3600).unwrap();
/// let base = beirut.with_ymd_and_hms(2024, 3, 15, 8, 0, 0).unwrap();
/// assert_eq!(EasyTime::weeks_before(base, 2), beirut.with_ymd_and_hms(2024, 3, 1, 8, 0, 0).unwrap());
/// ```
pub struct EasyTime<Tz: TimeZone>(PhantomData<Tz>);

impl<Tz: TimeZone> EasyTime<Tz> {
    /// Returns `base` moved by `amount` units (back if negative), or `None` if the result is out of range.
    ///
    /// ```rust
    /// use easy_time::{EasyTime, Unit};
    /// use chrono::{TimeZone, Utc};
    ///
    /// let leap_day = Utc.with_ymd_and_hms(2024, 2, 29, 0, 0, 0).unwrap();
    /// let next = EasyTime::checked_add(leap_day, 1, Unit::Years);
    /// assert_eq!(next, Some(Utc.with_ymd_and_hms(2025, 2, 28, 0, 0, 0).unwrap()));
    /// assert_eq!(EasyTime::checked_add(leap_day, i64::MAX, Unit::Days), None);
    /// ```
    #[must_use]
    pub fn checked_add(base: DateTime<Tz>, amount: i64, unit: Unit) -> Option<DateTime<Tz>> {
        if let Some(seconds) = unit.exact_seconds() {
            return base.checked_add_signed(TimeDelta::try_seconds(amount.checked_mul(seconds)?)?);
        }
        let moved = add_wall(base.naive_local(), amount, unit)?;
        resolve(&base.timezone(), moved, Some(base.offset()), false)
    }

    /// Returns the first instant of the `unit` that contains `time`: midnight for [`Unit::Days`],
    /// Monday midnight for [`Unit::Weeks`], the 1st at midnight for [`Unit::Months`], and so on.
    /// If that wall-clock time is skipped by DST, this is the instant the skipped stretch ends.
    ///
    /// ```rust
    /// use easy_time::{EasyTime, Unit};
    /// use chrono::{TimeZone, Utc};
    ///
    /// let thursday = Utc.with_ymd_and_hms(2024, 5, 16, 15, 42, 7).unwrap();
    /// assert_eq!(EasyTime::start_of(thursday, Unit::Weeks), Utc.with_ymd_and_hms(2024, 5, 13, 0, 0, 0).unwrap());
    /// ```
    ///
    /// # Panics
    ///
    /// If the result is outside chrono's range.
    #[must_use]
    pub fn start_of(time: DateTime<Tz>, unit: Unit) -> DateTime<Tz> {
        period_start(time.naive_local(), unit)
            .and_then(|start| resolve(&time.timezone(), start, prefer(&time, unit), true))
            .expect(OUT_OF_RANGE)
    }

    /// Returns the last instant (to the nanosecond) of the `unit` that contains `time`.
    ///
    /// ```rust
    /// use easy_time::{EasyTime, Unit};
    /// use chrono::{TimeZone, Timelike, Utc};
    ///
    /// let time = Utc.with_ymd_and_hms(2024, 2, 10, 15, 0, 0).unwrap();
    /// let end = EasyTime::end_of(time, Unit::Months);
    /// assert_eq!(end, Utc.with_ymd_and_hms(2024, 2, 29, 23, 59, 59).unwrap().with_nanosecond(999_999_999).unwrap());
    /// ```
    ///
    /// # Panics
    ///
    /// If the result is outside chrono's range.
    #[must_use]
    pub fn end_of(time: DateTime<Tz>, unit: Unit) -> DateTime<Tz> {
        let tz = time.timezone();
        let start = Self::start_of(time.clone(), unit);
        let in_period = |instant: DateTime<Tz>| Self::start_of(instant, unit) == start;
        let at = |second: i64| tz.timestamp_opt(second, 0).single().expect(OUT_OF_RANGE);
        // The next period usually starts one unit later on the wall clock ...
        let mut next = period_start(time.naive_local(), unit)
            .and_then(|start| add_wall(start, 1, unit))
            .and_then(|next| resolve(&tz, next, prefer(&time, unit), true))
            .expect(OUT_OF_RANGE);
        // ... but sooner if DST turns the clock back first: find the first second outside this period.
        if !in_period(next.clone() - TimeDelta::seconds(1)) {
            let (mut inside, mut outside) = (time.timestamp(), next.timestamp());
            while outside - inside > 1 {
                let mid = inside + (outside - inside) / 2;
                if in_period(at(mid)) {
                    inside = mid
                } else {
                    outside = mid
                }
            }
            next = at(outside);
        }
        next - TimeDelta::nanoseconds(1)
    }
}

impl Unit {
    /// Length in seconds of the units that are exact elapsed time; `None` for calendar units.
    const fn exact_seconds(self) -> Option<i64> {
        match self {
            Unit::Seconds => Some(1),
            Unit::Minutes => Some(60),
            Unit::Hours => Some(3600),
            _ => None,
        }
    }
}

/// The wall-clock time at which the `unit` containing `local` starts.
fn period_start(local: NaiveDateTime, unit: Unit) -> Option<NaiveDateTime> {
    let date = local.date();
    let year_start =
        |every: i32| NaiveDate::from_ymd_opt(date.year() - date.year().rem_euclid(every), 1, 1);
    let day = match unit {
        Unit::Seconds | Unit::Minutes | Unit::Hours | Unit::Days => Some(date),
        Unit::Weeks => {
            date.checked_sub_days(Days::new(date.weekday().num_days_from_monday().into()))
        }
        Unit::Months => date.with_day(1),
        Unit::Years => year_start(1),
        Unit::Decades => year_start(10),
        Unit::Centuries => year_start(100),
        Unit::Millenniums => year_start(1000),
    };
    let (hour, minute, second) = match unit {
        Unit::Seconds => (local.hour(), local.minute(), local.second()),
        Unit::Minutes => (local.hour(), local.minute(), 0),
        Unit::Hours => (local.hour(), 0, 0),
        _ => (0, 0, 0),
    };
    day?.and_hms_opt(hour, minute, second)
}

/// The DST overlap preference for the period of `unit` containing `time`: a repeated hour is two
/// separate hours (keep `time`'s offset), but a day with a repeated hour is still one day (take the earlier).
fn prefer<Tz: TimeZone>(time: &DateTime<Tz>, unit: Unit) -> Option<&Tz::Offset> {
    unit.exact_seconds().map(|_| time.offset())
}

/// Moves a wall-clock time by `amount` units, clamping the day to the target month's length.
fn add_wall(local: NaiveDateTime, amount: i64, unit: Unit) -> Option<NaiveDateTime> {
    let months = |per: i64| add_months(local, amount.checked_mul(per)?);
    match unit {
        Unit::Seconds | Unit::Minutes | Unit::Hours => {
            let seconds = amount.checked_mul(unit.exact_seconds()?)?;
            local.checked_add_signed(TimeDelta::try_seconds(seconds)?)
        }
        Unit::Days => add_days(local, amount),
        Unit::Weeks => add_days(local, amount.checked_mul(7)?),
        Unit::Months => months(1),
        Unit::Years => months(12),
        Unit::Decades => months(120),
        Unit::Centuries => months(1200),
        Unit::Millenniums => months(12_000),
    }
}

/// Generates the named shortcuts: `<unit>_from`/`<unit>_before` for any zone and
/// `<unit>_from_now`/`<unit>_ago` for zones that implement [`Now`].
macro_rules! unit_methods {
    ($($unit:ident $name:literal $from_now:ident $ago:ident $from:ident $before:ident)*) => {
        impl<Tz: TimeZone> EasyTime<Tz> {$(
            #[doc = concat!("Returns `base` plus `n` ", $name, ".\n\n# Panics\n\nIf the result is outside chrono's range.")]
            #[must_use]
            pub fn $from(base: DateTime<Tz>, n: i64) -> DateTime<Tz> {
                shifted(base, Some(n), Unit::$unit)
            }

            #[doc = concat!("Returns `base` minus `n` ", $name, ".\n\n# Panics\n\nIf the result is outside chrono's range.")]
            #[must_use]
            pub fn $before(base: DateTime<Tz>, n: i64) -> DateTime<Tz> {
                shifted(base, n.checked_neg(), Unit::$unit)
            }
        )*}

        impl<Tz: Now> EasyTime<Tz> {$(
            #[doc = concat!("Returns the time `n` ", $name, " from now.\n\n# Panics\n\nIf the result is outside chrono's range.")]
            #[must_use]
            pub fn $from_now(n: i64) -> DateTime<Tz> {
                Self::$from(Tz::now(), n)
            }

            #[doc = concat!("Returns the time `n` ", $name, " ago.\n\n# Panics\n\nIf the result is outside chrono's range.")]
            #[must_use]
            pub fn $ago(n: i64) -> DateTime<Tz> {
                Self::$before(Tz::now(), n)
            }
        )*}
    };
}

unit_methods! {
    Seconds "seconds" seconds_from_now seconds_ago seconds_from seconds_before
    Minutes "minutes" minutes_from_now minutes_ago minutes_from minutes_before
    Hours "hours" hours_from_now hours_ago hours_from hours_before
    Days "days" days_from_now days_ago days_from days_before
    Weeks "weeks" weeks_from_now weeks_ago weeks_from weeks_before
    Months "months" months_from_now months_ago months_from months_before
    Years "years" years_from_now years_ago years_from years_before
    Decades "decades" decades_from_now decades_ago decades_from decades_before
    Centuries "centuries" centuries_from_now centuries_ago centuries_from centuries_before
    Millenniums "millenniums" millenniums_from_now millenniums_ago millenniums_from millenniums_before
}

fn shifted<Tz: TimeZone>(base: DateTime<Tz>, amount: Option<i64>, unit: Unit) -> DateTime<Tz> {
    amount
        .and_then(|amount| EasyTime::checked_add(base, amount, unit))
        .expect(OUT_OF_RANGE)
}

fn add_days(local: NaiveDateTime, days: i64) -> Option<NaiveDateTime> {
    let step = Days::new(days.unsigned_abs());
    if days < 0 {
        local.checked_sub_days(step)
    } else {
        local.checked_add_days(step)
    }
}

fn add_months(local: NaiveDateTime, months: i64) -> Option<NaiveDateTime> {
    let step = Months::new(u32::try_from(months.unsigned_abs()).ok()?);
    if months < 0 {
        local.checked_sub_months(step)
    } else {
        local.checked_add_months(step)
    }
}

/// Turns a wall-clock time into a real instant. In a DST overlap it takes the later instant only if
/// it has the `prefer`red offset. In a DST gap it moves forward by the length of the gap or, with
/// `to_gap_end`, to the instant the gap ends.
fn resolve<Tz: TimeZone>(
    tz: &Tz,
    local: NaiveDateTime,
    prefer: Option<&Tz::Offset>,
    to_gap_end: bool,
) -> Option<DateTime<Tz>> {
    match tz.from_local_datetime(&local) {
        MappedLocalTime::Single(time) => Some(time),
        MappedLocalTime::Ambiguous(earlier, later) => {
            let keep_later = prefer.is_some_and(|offset| offset.fix() == later.offset().fix());
            Some(if keep_later { later } else { earlier })
        }
        MappedLocalTime::None => {
            // Reading the wall time with the offset in force before the gap lands past it, by the gap's length.
            let day_before = local.checked_sub_days(Days::new(1))?;
            let offset = tz.offset_from_local_datetime(&day_before).earliest()?.fix();
            let shifted = tz.from_utc_datetime(&local.checked_sub_offset(offset)?);
            if !to_gap_end {
                return Some(shifted);
            }
            // The gap ends at the first instant whose wall time is at or after `local`. Reading `local`
            // with the offset after the gap gives an instant before the gap; binary-search the seconds between.
            let mut before = local
                .checked_sub_offset(shifted.offset().fix())?
                .and_utc()
                .timestamp();
            let mut after = shifted.timestamp();
            while after - before > 1 {
                let mid = before + (after - before) / 2;
                if tz.timestamp_opt(mid, 0).single()?.naive_local() >= local {
                    after = mid;
                } else {
                    before = mid;
                }
            }
            tz.timestamp_opt(after, 0).single()
        }
    }
}

// ----------------------------------------------------------
//                     Calendar helpers
// ----------------------------------------------------------

/// Returns whether `year` is a leap year in the proleptic Gregorian calendar.
///
/// ```rust
/// assert!(easy_time::is_leap_year(2024));
/// assert!(!easy_time::is_leap_year(1900));
/// ```
#[must_use]
pub const fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// Returns the number of days in `month` (1 to 12) of `year`, or `None` for any other month.
///
/// ```rust
/// assert_eq!(easy_time::days_in_month(2024, 2), Some(29));
/// assert_eq!(easy_time::days_in_month(2024, 13), None);
/// ```
#[must_use]
pub const fn days_in_month(year: i32, month: u32) -> Option<u32> {
    match month {
        2 if is_leap_year(year) => Some(29),
        2 => Some(28),
        4 | 6 | 9 | 11 => Some(30),
        1..=12 => Some(31),
        _ => None,
    }
}

// ----------------------------------------------------------
//                     Human-readable output
// ----------------------------------------------------------

/// Describes a span in words, counting the largest whole unit: `"in 2 hours"` for a positive
/// span, `"3 days ago"` for a negative one, and `"just now"` for under half a second.
///
/// A month counts as 30 days and a year as 365.
///
/// ```rust
/// use chrono::TimeDelta;
/// use easy_time::humanize_delta;
///
/// assert_eq!(humanize_delta(TimeDelta::minutes(90)), "in 1 hour");
/// assert_eq!(humanize_delta(TimeDelta::days(-14)), "2 weeks ago");
/// ```
#[must_use]
pub fn humanize_delta(delta: TimeDelta) -> String {
    const UNITS: [(u64, &str); 7] = [
        (365 * 86_400, "year"),
        (30 * 86_400, "month"),
        (7 * 86_400, "week"),
        (86_400, "day"),
        (3_600, "hour"),
        (60, "minute"),
        (1, "second"),
    ];
    // Rounding to the nearest second first keeps "3 days from now" from reading as "in 2 days".
    let millis = delta.num_milliseconds();
    let seconds = (millis.unsigned_abs() + 500) / 1000;
    let Some(&(size, name)) = UNITS.iter().find(|(size, _)| seconds >= *size) else {
        return "just now".to_owned();
    };
    let count = seconds / size;
    let words = format!("{count} {name}{}", if count == 1 { "" } else { "s" });
    if millis > 0 {
        format!("in {words}")
    } else {
        format!("{words} ago")
    }
}

/// Describes `time` relative to now, like [`humanize_delta`]: `"3 days ago"`, `"in 2 hours"`.
#[must_use]
pub fn humanize<Tz: TimeZone>(time: &DateTime<Tz>) -> String {
    humanize_delta(time.clone().signed_duration_since(Utc::now()))
}

/// Formats a datetime as `YYYY-MM-DD HH:MM:SS`.
#[must_use]
pub fn format_datetime<Tz: TimeZone>(time: &DateTime<Tz>) -> String
where
    Tz::Offset: Display,
{
    time.format(DEFAULT_DATE_FORMAT).to_string()
}

/// Formats a datetime with a [chrono format string](chrono::format::strftime),
/// or returns `None` if the format string is invalid.
///
/// ```rust
/// use chrono::{TimeZone, Utc};
/// use easy_time::format_datetime_with;
///
/// let time = Utc.with_ymd_and_hms(2024, 7, 4, 9, 5, 0).unwrap();
/// assert_eq!(format_datetime_with(&time, "%d/%m/%Y %H:%M").as_deref(), Some("04/07/2024 09:05"));
/// assert_eq!(format_datetime_with(&time, "%Q"), None);
/// ```
#[must_use]
pub fn format_datetime_with<Tz: TimeZone>(time: &DateTime<Tz>, format: &str) -> Option<String>
where
    Tz::Offset: Display,
{
    let mut out = String::new();
    write!(out, "{}", time.format(format)).ok()?;
    Some(out)
}

/// Formats a datetime as `YYYY-MM-DD HH:MM:SS +HH:MM`.
#[must_use]
pub fn format_datetime_with_timezone<Tz: TimeZone>(time: &DateTime<Tz>) -> String
where
    Tz::Offset: Display,
{
    time.format("%Y-%m-%d %H:%M:%S %:z").to_string()
}

/// Returns just the date portion as `YYYY-MM-DD`.
#[must_use]
pub fn to_date<Tz: TimeZone>(time: &DateTime<Tz>) -> String
where
    Tz::Offset: Display,
{
    time.format(DATE_FORMAT).to_string()
}

/// Returns just the time portion as `HH:MM:SS`.
#[must_use]
pub fn to_time<Tz: TimeZone>(time: &DateTime<Tz>) -> String
where
    Tz::Offset: Display,
{
    time.format(TIME_FORMAT).to_string()
}
