use chrono::prelude::{DateTime, TimeZone};
use chrono::{Datelike, Duration, Local, LocalResult, Utc};

// create enum for seconds, minutes, hours, days, months, years, decades, centuries, millenniums
#[derive(Debug)]
pub enum TimeUnits {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
    Decades,
    Centuries,
    Millenniums,
}

#[derive(Clone, PartialEq, Debug, Eq)]
pub struct EasyTime<F: TimeZone> {
    pub value: i64,
    pub time_now: DateTime<F>,
}

// ----------------------------------------------------------
//           EasyTime<Local>: Constructors
// ----------------------------------------------------------
impl EasyTime<Local> {
    pub fn new(value: i64) -> Self {
        Self {
            value,
            time_now: Local::now(),
        }
    }

    pub fn new_with_local(time: DateTime<Local>, value: i64) -> Self {
        Self {
            value,
            time_now: time,
        }
    }

    // value then type of time unit then time or if time is not provided then current time
    pub fn in_future(
        value: i64,
        time_unit: TimeUnits,
        time: Option<DateTime<Local>>,
    ) -> DateTime<Local> {
        let time = time.unwrap_or(Local::now());
        let easy_time = EasyTime::new_with_local(time, value);
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

    pub fn in_past(
        value: i64,
        time_unit: TimeUnits,
        time: Option<DateTime<Local>>,
    ) -> DateTime<Local> {
        let time = time.unwrap_or(Local::now());
        let easy_time = EasyTime::new_with_local(time, value);
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
//           EasyTime<Utc>: Constructors
// ----------------------------------------------------------
impl EasyTime<Utc> {
    pub fn new_with_utc(value: i64) -> Self {
        Self {
            value,
            time_now: Utc::now(),
        }
    }

    pub fn new_with_utc_time(time: DateTime<Utc>, value: i64) -> Self {
        Self {
            value,
            time_now: time,
        }
    }

    // value then type of time unit then time or if time is not provided then current time

    pub fn in_future(
        value: i64,
        time_unit: TimeUnits,
        time: Option<DateTime<Utc>>,
    ) -> DateTime<Utc> {
        let time = time.unwrap_or(Utc::now());
        let easy_time = EasyTime::new_with_utc_time(time, value);
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

    pub fn in_past(value: i64, time_unit: TimeUnits, time: Option<DateTime<Utc>>) -> DateTime<Utc> {
        let time = time.unwrap_or(Utc::now());
        let easy_time = EasyTime::new_with_utc_time(time, value);
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
//   EasyTime<F> for Any TimeZone: Generic Implementation
// ----------------------------------------------------------
impl<F: TimeZone> EasyTime<F>
where
    F::Offset: std::fmt::Display,
{
    pub fn from_time(time: DateTime<F>) -> Self {
        Self {
            value: 0,
            time_now: time,
        }
    }

    /// Creates an `EasyTime` with a specific `time_now` in any timezone `F`.
    pub fn new_with_time(value: i64, time: DateTime<F>) -> Self {
        Self {
            value,
            time_now: time,
        }
    }

    // ------------------------------------------------------------------
    //                    Getters / Setters
    // ------------------------------------------------------------------
    pub fn set_value(&mut self, value: i64) {
        self.value = value;
    }

    pub fn set_time(&mut self, time: DateTime<F>) {
        self.time_now = time;
    }

    pub fn get_value(&self) -> i64 {
        self.value
    }

    pub fn get_time(&self) -> DateTime<F> {
        self.time_now.clone()
    }

    // ------------------------------------------------------------------
    //                      Internal Helpers
    // ------------------------------------------------------------------
    fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    fn days_in_month(year: i32, month: u32) -> u32 {
        match month {
            1 => 31,
            2 => {
                if Self::is_leap_year(year) {
                    29
                } else {
                    28
                }
            }
            3 => 31,
            4 => 30,
            5 => 31,
            6 => 30,
            7 => 31,
            8 => 31,
            9 => 30,
            10 => 31,
            11 => 30,
            12 => 31,
            _ => panic!("Invalid month"),
        }
    }

    // /// Add `duration` to `time_now`.
    // pub fn offset(&self, duration: Duration) -> DateTime<F> {
    //     self.time_now.clone() + duration
    // }

    // /// Subtract `duration` from `time_now`.
    // pub fn offset_neg(&self, duration: Duration) -> DateTime<F> {
    //     self.time_now.clone() - duration
    // }

    pub fn offset(time: DateTime<F>, duration: Duration) -> DateTime<F> {
        time + duration
    }

    pub fn offset_neg(time: DateTime<F>, duration: Duration) -> DateTime<F> {
        time - duration
    }

    /// Tries to build a `DateTime<F>` from a naive date-time.
    /// Uses `.unwrap()` in ambiguous cases by picking the first match, and panics on invalid.
    fn from_naive_local(&self, naive: chrono::NaiveDateTime) -> DateTime<F> {
        match self.time_now.timezone().from_local_datetime(&naive) {
            LocalResult::Single(dt) => dt,
            LocalResult::Ambiguous(a, _b) => a,
            LocalResult::None => panic!("Invalid or non-existent local time."),
        }
    }

    // ------------------------------------------------------------------
    //           Simple Offsets: seconds, minutes, hours, days
    // ------------------------------------------------------------------
    pub fn seconds_from_now(&self) -> DateTime<F> {
        Self::offset(self.time_now.clone(), Duration::seconds(self.value))
    }

    pub fn seconds_ago(&self) -> DateTime<F> {
        Self::offset_neg(self.time_now.clone(), Duration::seconds(self.value))
    }

    pub fn minutes_from_now(&self) -> DateTime<F> {
        Self::offset(self.time_now.clone(), Duration::minutes(self.value))
    }

    pub fn minutes_ago(&self) -> DateTime<F> {
        Self::offset_neg(self.time_now.clone(), Duration::minutes(self.value))
    }

    pub fn hours_from_now(&self) -> DateTime<F> {
        Self::offset(self.time_now.clone(), Duration::hours(self.value))
    }

    pub fn hours_ago(&self) -> DateTime<F> {
        Self::offset_neg(self.time_now.clone(), Duration::hours(self.value))
    }

    pub fn days_from_now(&self) -> DateTime<F> {
        Self::offset(self.time_now.clone(), Duration::days(self.value))
    }

    pub fn days_ago(&self) -> DateTime<F> {
        Self::offset_neg(self.time_now.clone(), Duration::days(self.value))
    }

    // ------------------------------------------------------------------
    //               Month-Based Offset (custom logic)
    // ------------------------------------------------------------------
    fn add_months(&self, months: i32) -> DateTime<F> {
        let naive = self.time_now.naive_local();
        let (year, month, day) = (naive.year(), naive.month() as i32, naive.day());

        // Calculate target year and month
        let mut target_year = year + (months / 12);
        let mut target_month = month + (months % 12);

        while target_month > 12 {
            target_month -= 12;
            target_year += 1;
        }
        while target_month < 1 {
            target_month += 12;
            target_year -= 1;
        }

        let days_in_target = Self::days_in_month(target_year, target_month as u32);
        let target_day = std::cmp::min(day, days_in_target);

        let target_date =
            chrono::NaiveDate::from_ymd_opt(target_year, target_month as u32, target_day)
                .expect("Invalid date after adding months");

        let target_naive_dt = target_date.and_time(naive.time());
        self.from_naive_local(target_naive_dt)
    }

    pub fn months_from_now(&self) -> DateTime<F> {
        self.add_months(self.value as i32)
    }

    pub fn months_ago(&self) -> DateTime<F> {
        self.add_months(-(self.value as i32))
    }

    // ------------------------------------------------------------------
    //               Year-Based Offsets (custom logic)
    // ------------------------------------------------------------------
    fn add_years(&self, years: i32) -> DateTime<F> {
        let naive = self.time_now.naive_local();
        let (year, month, day) = (naive.year() + years, naive.month(), naive.day());

        let days_in_target = Self::days_in_month(year, month);
        let target_day = std::cmp::min(day, days_in_target);

        let target_date = chrono::NaiveDate::from_ymd_opt(year, month, target_day)
            .expect("Invalid date after adding years");

        let target_naive_dt = target_date.and_time(naive.time());
        self.from_naive_local(target_naive_dt)
    }

    pub fn years_from_now(&self) -> DateTime<F> {
        self.add_years(self.value as i32)
    }

    pub fn years_ago(&self) -> DateTime<F> {
        self.add_years(-(self.value as i32))
    }

    pub fn decades_from_now(&self) -> DateTime<F> {
        self.add_years(self.value as i32 * 10)
    }

    pub fn decades_ago(&self) -> DateTime<F> {
        self.add_years(-(self.value as i32) * 10)
    }

    pub fn centuries_from_now(&self) -> DateTime<F> {
        self.add_years(self.value as i32 * 100)
    }

    pub fn centuries_ago(&self) -> DateTime<F> {
        self.add_years(-(self.value as i32) * 100)
    }

    pub fn millenniums_from_now(&self) -> DateTime<F> {
        self.add_years(self.value as i32 * 1000)
    }

    pub fn millenniums_ago(&self) -> DateTime<F> {
        self.add_years(-(self.value as i32) * 1000)
    }

    // ------------------------------------------------------------------
    //          Formatting Methods
    // ------------------------------------------------------------------
    /// Internal helper to format the current time with an optional timezone suffix.
    fn format_with(&self, format_str: &str, show_tz: bool) -> String {
        let base = self.time_now.format(format_str).to_string();
        if show_tz {
            format!("{} {}", base, self.time_now.offset())
        } else {
            base
        }
    }

    pub fn to_string(&self) -> String {
        self.format_with("%Y-%m-%d %H:%M:%S", false)
    }

    pub fn to_string_with_format(&self, format_str: &str) -> String {
        self.format_with(format_str, false)
    }

    pub fn to_string_with_timezone(&self) -> String {
        self.format_with("%Y-%m-%d %H:%M:%S", true)
    }

    pub fn to_string_with_timezone_format(&self, format_str: &str) -> String {
        self.format_with(format_str, true)
    }

    pub fn to_string_with_timezone_format_with_timezone(&self, format_str: &str) -> String {
        // Essentially the same as the above, but kept for backward compatibility
        self.format_with(format_str, true)
    }

    // ------------------------------------------------------------------
    //           Other Utilities
    // ------------------------------------------------------------------
    pub fn to_timestamp(&self) -> i64 {
        self.time_now.timestamp()
    }

    pub fn to_date(&self) -> String {
        self.time_now.format("%Y-%m-%d").to_string()
    }

    pub fn to_time(&self) -> String {
        self.time_now.format("%H:%M:%S").to_string()
    }

    pub fn to_date_time(&self) -> String {
        self.time_now.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    pub fn to_date_time_with_timezone_format(&self, format_str: &str) -> String {
        format!(
            "{} {}",
            self.time_now.format(format_str),
            self.time_now.offset()
        )
    }
}
