use chrono::prelude::DateTime;
use chrono::TimeZone;
use chrono::{Datelike, Duration, Local, Utc};

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

    fn days_between_years(start_year: i64, end_year: i64) -> i64 {
        let mut days: i64 = 0;
        for year in start_year..end_year {
            if year % 4 == 0 {
                days += 366;
            } else {
                days += 365;
            }
        }
        days
    }

    fn days_between_months(start_month: i64, end_month: i64, year: i64) -> i64 {
        let mut days: i64 = 0;
        for month in start_month..end_month {
            days += match month {
                1 => 31,
                2 => {
                    if year % 4 == 0 {
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
                _ => 0,
            }
        }
        days
    }

    fn months_handler(date: DateTime<F>, months_count: i64) -> i64 {
        let current_month: i64 = date.month().into();
        let current_year: i64 = date.year().into();
        let years_count = months_count / 12;
        let to_year: i64 = current_year + years_count;
        let months_count = months_count % 12;
        let mut days_count = 0;
        if months_count > 0 {
            days_count += Self::days_between_months(current_month, months_count, to_year);
        }
        if years_count > 0 {
            days_count += Self::days_between_years(current_year, to_year)
        }
        days_count
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
    pub fn months_from_now(&self) -> DateTime<F> {
        let days_count: i64 = Self::months_handler(self.time_now.clone(), self.value);
        self.time_now.clone() + Duration::days(days_count)
    }

    pub fn months_ago(&self) -> DateTime<F> {
        let days_count: i64 = Self::months_handler(self.time_now.clone(), self.value);
        self.time_now.clone() - Duration::days(days_count)
    }

    pub fn years_from_now(&self) -> DateTime<F> {
        let current_year: i64 = self.time_now.clone().year().into();
        let target_year: i64 = current_year + self.value;
        let day_count: i64 = Self::days_between_years(current_year, target_year);
        self.time_now.clone() + Duration::days(day_count)
    }

    pub fn years_ago(&self) -> DateTime<F> {
        let current_year: i64 = self.time_now.clone().year().into();
        let target_year: i64 = current_year - self.value;
        let day_count: i64 = Self::days_between_years(target_year, current_year);
        self.time_now.clone() - Duration::days(day_count)
    }

    pub fn decades_from_now(&self) -> DateTime<F> {
        let current_year: i64 = self.time_now.clone().year().into();
        let target_year: i64 = current_year + (self.value * 10);
        let day_count: i64 = Self::days_between_years(current_year, target_year);
        self.time_now.clone() + Duration::days(day_count)
    }

    pub fn decades_ago(&self) -> DateTime<F> {
        let current_year: i64 = self.time_now.clone().year().into();
        let target_year: i64 = current_year - (self.value * 10);
        let day_count: i64 = Self::days_between_years(target_year, current_year);
        self.time_now.clone() - Duration::days(day_count)
    }

    pub fn centuries_from_now(&self) -> DateTime<F> {
        let current_year: i64 = self.time_now.clone().year().into();
        let target_year: i64 = current_year + (self.value * 100);
        let day_count: i64 = Self::days_between_years(current_year, target_year);
        self.time_now.clone() + Duration::days(day_count)
    }

    pub fn centuries_ago(&self) -> DateTime<F> {
        let current_year: i64 = self.time_now.clone().year().into();
        let target_year: i64 = current_year - (self.value * 100);
        let day_count: i64 = Self::days_between_years(target_year, current_year);
        self.time_now.clone() - Duration::days(day_count)
    }


    pub fn milleniums_from_now(&self) -> DateTime<F> {
        let current_year: i64 = self.time_now.clone().year().into();
        let target_year: i64 = current_year + (self.value * 1000);
        let day_count: i64 = Self::days_between_years(current_year, target_year);
        self.time_now.clone() + Duration::days(day_count)
    }


    pub fn milleniums_ago(&self) -> DateTime<F> {
        let current_year: i64 = self.time_now.clone().year().into();
        let target_year: i64 = current_year - (self.value * 1000);
        let day_count: i64 = Self::days_between_years(target_year, current_year);
        self.time_now.clone() - Duration::days(day_count)
    }
}
