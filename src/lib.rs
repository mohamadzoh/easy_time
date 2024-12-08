use chrono::prelude::*;
use chrono::{Datelike, Duration, Local, TimeZone, Utc};

pub struct EasyTime<F: TimeZone> {
    pub value: i64,
    pub time_now: DateTime<F>,
}

impl<F: TimeZone> EasyTime<F> {
    pub fn new(value: i64) -> EasyTime<Local> {
        EasyTime {
            value,
            time_now: Local::now(),
        }
    }

    pub fn new_with_utc(value: i64) -> EasyTime<Utc> {
        EasyTime {
            value,
            time_now: Utc::now(),
        }
    }

    pub fn new_with_time(value: i64, time: DateTime<F>) -> EasyTime<F> {
        EasyTime {
            value,
            time_now: time,
        }
    }

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

    pub fn seconds_from_now(&self) -> DateTime<F> {
        self.time_now.clone() + Duration::seconds(self.value)
    }

    pub fn seconds_ago(&self) -> DateTime<F> {
        self.time_now.clone() - Duration::seconds(self.value)
    }

    pub fn minutes_from_now(&self) -> DateTime<F> {
        self.time_now.clone() + Duration::minutes(self.value)
    }

    pub fn minutes_ago(&self) -> DateTime<F> {
        self.time_now.clone() - Duration::minutes(self.value)
    }

    pub fn hours_from_now(&self) -> DateTime<F> {
        self.time_now.clone() + Duration::hours(self.value)
    }

    pub fn hours_ago(&self) -> DateTime<F> {
        self.time_now.clone() - Duration::hours(self.value)
    }

    pub fn days_from_now(&self) -> DateTime<F> {
        self.time_now.clone() + Duration::days(self.value)
    }

    pub fn days_ago(&self) -> DateTime<F> {
        self.time_now.clone() - Duration::days(self.value)
    }

    /// Adjusts the date by a certain number of months.
    /// This method tries to keep the day of the month stable. If the target month doesn't have the current day,
    /// it uses the last valid day of the target month.
    fn add_months(&self, months: i32) -> DateTime<F> {
        let naive = self.time_now.naive_local();
        let year = naive.year();
        let month = naive.month() as i32;
        let day = naive.day();

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

        let days_in_target_month = Self::days_in_month(target_year, target_month as u32);
        let target_day = std::cmp::min(day, days_in_target_month);

        let target_date = chrono::NaiveDate::from_ymd_opt(target_year, target_month as u32, target_day)
            .expect("Invalid date");
        let target_naive_dt = target_date.and_time(naive.time());

        // Convert back to the original timezone
        self.time_now.timezone().from_local_datetime(&target_naive_dt).unwrap()
    }

    pub fn months_from_now(&self) -> DateTime<F> {
        self.add_months(self.value as i32)
    }

    pub fn months_ago(&self) -> DateTime<F> {
        self.add_months(-(self.value as i32))
    }

    /// Similar logic for adding years
    fn add_years(&self, years: i32) -> DateTime<F> {
        let naive = self.time_now.naive_local();
        let year = naive.year() + years;
        let month = naive.month();
        let day = naive.day();

        let days_in_target_month = Self::days_in_month(year, month);
        let target_day = std::cmp::min(day, days_in_target_month);

        let target_date = chrono::NaiveDate::from_ymd_opt(year, month, target_day)
            .expect("Invalid date");
        let target_naive_dt = target_date.and_time(naive.time());
        self.time_now.timezone().from_local_datetime(&target_naive_dt).unwrap()
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
}
