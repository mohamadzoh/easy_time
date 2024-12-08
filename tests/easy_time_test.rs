extern crate easy_time;
use chrono::prelude::Local;
use chrono::Duration;
use easy_time::EasyTime;


#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    // Test the EasyTime::new method
    #[test]
    fn test_easy_time_new() {
        let easy_time: EasyTime<Local> = EasyTime::<Local>::new(10);
        assert_eq!(easy_time.value, 10);
    }

    // Test the EasyTime::seconds_from_now method
    #[test]
    fn test_easy_time_seconds_from_now() {
        let date_time: chrono::DateTime<Local> = Local::now();
        let easy_time: EasyTime<Local> = EasyTime::new_with_time(10, date_time);
        let expected: chrono::DateTime<Local> = date_time + Duration::seconds(10);
        // assert_eq!(easy_time.seconds_from_now(), expected);
        assert_eq!(easy_time.seconds_from_now(),expected);
    }

    // Test the EasyTime::minutes_from_now method
    #[test]
    fn test_easy_time_minutes_from_now() {
        let date_time: chrono::DateTime<Local> = Local::now();
        let easy_time: EasyTime<Local> = EasyTime::new_with_time(10, date_time);
        let expected: chrono::DateTime<Local> = date_time + Duration::minutes(10);
        assert_eq!(easy_time.minutes_from_now(), expected);
    }

    // Test the EasyTime::hours_from_now method
    #[test]
    fn test_easy_time_hours_from_now() {
        let date_time = Local::now();
        let easy_time = EasyTime::new_with_time(10, date_time);
        let expected = date_time + Duration::hours(10);
        assert_eq!(easy_time.hours_from_now(), expected);
    }

    // Test the EasyTime::days_from_now method
    #[test]
    fn test_easy_time_days_from_now() {
        let date_time: chrono::DateTime<Local> = Local::now();
        let easy_time: EasyTime<Local> = EasyTime::new_with_time(10, date_time);
        let expected: chrono::DateTime<Local> = date_time + Duration::days(10);
        assert_eq!(easy_time.days_from_now(), expected);
    }

    // Test the EasyTime::seconds_ago method
    #[test]
    fn test_easy_time_seconds_ago() {
        let date_time: chrono::DateTime<Local> = Local::now();
        let easy_time: EasyTime<Local> = EasyTime::new_with_time(10, date_time);
        let expected: chrono::DateTime<Local> = date_time - Duration::seconds(10);
        assert_eq!(easy_time.seconds_ago(), expected);
    }

    // Test the EasyTime::minutes_ago method
    #[test]
    fn test_easy_time_minutes_ago() {
        let date_time: chrono::DateTime<Local> = Local::now();
        let easy_time: EasyTime<Local> = EasyTime::new_with_time(10, date_time);
        let expected: chrono::DateTime<Local> = date_time - Duration::minutes(10);
        assert_eq!(easy_time.minutes_ago(), expected);
    }

    // Test the EasyTime::hours_ago method
    #[test]
    fn test_easy_time_hours_ago() {
        let date_time: chrono::DateTime<Local> = Local::now();
        let easy_time: EasyTime<Local> = EasyTime::new_with_time(10, date_time);
        let expected: chrono::DateTime<Local> = date_time - Duration::hours(10);
        assert_eq!(easy_time.hours_ago(), expected);
    }

    // Test the EasyTime::days_ago method
    #[test]
    fn test_easy_time_days_ago() {
        let date_time: chrono::DateTime<Local> = Local::now();
        let easy_time: EasyTime<Local> = EasyTime::new_with_time(10, date_time);
        let expected: chrono::DateTime<Local> = date_time - Duration::days(10);
        assert_eq!(easy_time.days_ago(), expected);
    }

    // Test the EasyTime::new_with_time method
    #[test]
    fn test_easy_time_new_with_time() {
        let date_time: chrono::DateTime<Local> = Local::now();
        let easy_time: EasyTime<Local> = EasyTime::new_with_time(10, date_time);
        assert_eq!(easy_time.value, 10);
        assert_eq!(easy_time.time_now, date_time);
    }

    #[test]
    fn test_months_from_now_crossing_boundary() {
        // Start at January 31st
        let date_time = Local.with_ymd_and_hms(2023, 1, 31, 12, 0, 0).unwrap();
        let easy_time = EasyTime::new_with_time(1, date_time);
        // Adding 1 month from January 31 should yield February 28 (non-leap year)
        let expected = Local.with_ymd_and_hms(2023, 2, 28, 12, 0, 0).unwrap();
        assert_eq!(easy_time.months_from_now(), expected);
    }

    // Test months_from_now during a leap year
    #[test]
    fn test_months_from_now_leap_year() {
        // Start at January 31st of a leap year (e.g., 2024)
        let date_time = Local.with_ymd_and_hms(2024, 1, 31, 12, 0, 0).unwrap();
        let easy_time = EasyTime::new_with_time(1, date_time);
        // Adding 1 month from January 31 on a leap year should yield February 29, 2024
        let expected = Local.with_ymd_and_hms(2024, 2, 29, 12, 0, 0).unwrap();
        assert_eq!(easy_time.months_from_now(), expected);
    }

    // Test months_ago crossing a year boundary
    #[test]
    fn test_months_ago_crossing_year_boundary() {
        // Start at March 15, 2023
        let date_time = Local.with_ymd_and_hms(2023, 3, 15, 10, 0, 0).unwrap();
        let easy_time = EasyTime::new_with_time(4, date_time);
        // Subtracting 4 months from March 15, 2023, should give November 15, 2022
        let expected = Local.with_ymd_and_hms(2022, 11, 15, 10, 0, 0).unwrap();
        assert_eq!(easy_time.months_ago(), expected);
    }

    // Test years_from_now with a leap year transition
    #[test]
    fn test_years_from_now_leap_transition() {
        // Start at Feb 29, 2024 (a leap year day)
        let date_time = Local.with_ymd_and_hms(2024, 2, 29, 12, 0, 0).unwrap();
        let easy_time = EasyTime::new_with_time(1, date_time);
        // Adding 1 year to Feb 29, 2024 should result in Feb 29 -> Feb 29 doesn't exist in 2025
        // It should clamp to Feb 28, 2025.
        let expected = Local.with_ymd_and_hms(2025, 2, 28, 12, 0, 0).unwrap();
        assert_eq!(easy_time.years_from_now(), expected);
    }

    // Test years_ago beyond a leap year
    #[test]
    fn test_years_ago_leap_transition() {
        // Start at Feb 29, 2024
        let date_time = Local.with_ymd_and_hms(2024, 2, 29, 12, 0, 0).unwrap();
        let easy_time = EasyTime::new_with_time(1, date_time);
        // Going 1 year ago from Feb 29, 2024 (leap day) would be Feb 28, 2023 (since Feb 29 doesn't exist in 2023)
        let expected = Local.with_ymd_and_hms(2023, 2, 28, 12, 0, 0).unwrap();
        assert_eq!(easy_time.years_ago(), expected);
    }

    // Test negative values directly with months_from_now (e.g., going backwards in time)
    #[test]
    fn test_negative_months_from_now() {
        // If value is negative, months_from_now should effectively go backwards in time.
        // For example, value = -2 and starting from March 15, 2023 should yield January 15, 2023
        let date_time = Local.with_ymd_and_hms(2023, 3, 15, 12, 0, 0).unwrap();
        let easy_time = EasyTime::new_with_time(-2, date_time);
        let expected = Local.with_ymd_and_hms(2023, 1, 15, 12, 0, 0).unwrap();
        assert_eq!(easy_time.months_from_now(), expected);
    }

    // Test larger offsets: decades_from_now
    #[test]
    fn test_decades_from_now() {
        let date_time = Local.with_ymd_and_hms(2023, 7, 15, 8, 30, 0).unwrap();
        let easy_time = EasyTime::new_with_time(2, date_time);
        // Adding 2 decades = 20 years
        let expected_year = 2023 + 20;
        // Attempting to preserve day/month: July 15, 2043
        let expected = Local.with_ymd_and_hms(expected_year, 7, 15, 8, 30, 0).unwrap();
        assert_eq!(easy_time.decades_from_now(), expected);
    }

    // Test centuries_from_now
    #[test]
    fn test_centuries_from_now() {
        let date_time = Local.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
        let easy_time = EasyTime::new_with_time(1, date_time);
        // Adding 1 century = 100 years
        let expected_year = 2100;
        let expected = Local.with_ymd_and_hms(expected_year, 1, 1, 0, 0, 0).unwrap();
        assert_eq!(easy_time.centuries_from_now(), expected);
    }

    // Test millenniums_from_now
    #[test]
    fn test_millenniums_from_now() {
        let date_time = Local.with_ymd_and_hms(1000, 6, 1, 12, 0, 0).unwrap();
        let easy_time = EasyTime::new_with_time(1, date_time);
        // Adding 1 millennium = 1000 years
        let expected_year = 2000;
        let expected = Local.with_ymd_and_hms(expected_year, 6, 1, 12, 0, 0).unwrap();
        assert_eq!(easy_time.millenniums_from_now(), expected);
    }

    #[test]
    fn test_easy_time_new_with_utc() {
        let easy_time: EasyTime<Utc> = EasyTime::<Utc>::new_with_utc(30); // Explicit type annotation
        assert_eq!(easy_time.value, 30);
        assert!(easy_time.time_now.timezone() == Utc);
    }
    // Test with large negative years (checking behavior far in the past)
    #[test]
    fn test_years_ago_large_negative() {
        let date_time = Local.with_ymd_and_hms(2023, 3, 1, 0, 0, 0).unwrap();
        let easy_time: EasyTime<Local> = EasyTime::new_with_time(100, date_time);
        // Going 100 years ago from 2023-03-01 => 1923-03-01
        let expected = Local.with_ymd_and_hms(1923, 3, 1, 0, 0, 0).unwrap();
        assert_eq!(easy_time.years_ago(), expected);
    }

}
