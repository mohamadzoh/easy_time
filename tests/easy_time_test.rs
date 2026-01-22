extern crate easy_time;
use chrono::prelude::Local;
use chrono::Duration;
use easy_time::EasyTime;

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    // Test the EasyTime::seconds_from_now method
    #[test]
    fn test_easy_time_seconds_from_now() {
        let before = Local::now();
        let result = EasyTime::<Local>::seconds_from_now(10);
        let after = Local::now();
        
        // Result should be approximately 10 seconds from now
        assert!(result >= before + Duration::seconds(10));
        assert!(result <= after + Duration::seconds(10));
    }

    // Test the EasyTime::seconds_from (with base time)
    #[test]
    fn test_easy_time_seconds_from() {
        let date_time = Local::now();
        let result = EasyTime::<Local>::seconds_from(date_time, 10);
        let expected = date_time + Duration::seconds(10);
        assert_eq!(result, expected);
    }

    // Test the EasyTime::minutes_from_now method
    #[test]
    fn test_easy_time_minutes_from_now() {
        let before = Local::now();
        let result = EasyTime::<Local>::minutes_from_now(10);
        let after = Local::now();
        
        assert!(result >= before + Duration::minutes(10));
        assert!(result <= after + Duration::minutes(10));
    }

    // Test the EasyTime::minutes_from (with base time)
    #[test]
    fn test_easy_time_minutes_from() {
        let date_time = Local::now();
        let result = EasyTime::<Local>::minutes_from(date_time, 10);
        let expected = date_time + Duration::minutes(10);
        assert_eq!(result, expected);
    }

    // Test the EasyTime::hours_from_now method
    #[test]
    fn test_easy_time_hours_from_now() {
        let before = Local::now();
        let result = EasyTime::<Local>::hours_from_now(10);
        let after = Local::now();
        
        assert!(result >= before + Duration::hours(10));
        assert!(result <= after + Duration::hours(10));
    }

    // Test the EasyTime::hours_from (with base time)
    #[test]
    fn test_easy_time_hours_from() {
        let date_time = Local::now();
        let result = EasyTime::<Local>::hours_from(date_time, 10);
        let expected = date_time + Duration::hours(10);
        assert_eq!(result, expected);
    }

    // Test the EasyTime::days_from_now method
    #[test]
    fn test_easy_time_days_from_now() {
        let before = Local::now();
        let result = EasyTime::<Local>::days_from_now(10);
        let after = Local::now();
        
        assert!(result >= before + Duration::days(10));
        assert!(result <= after + Duration::days(10));
    }

    // Test the EasyTime::days_from (with base time)
    #[test]
    fn test_easy_time_days_from() {
        let date_time = Local::now();
        let result = EasyTime::<Local>::days_from(date_time, 10);
        let expected = date_time + Duration::days(10);
        assert_eq!(result, expected);
    }

    // Test the EasyTime::seconds_ago method
    #[test]
    fn test_easy_time_seconds_ago() {
        let before = Local::now();
        let result = EasyTime::<Local>::seconds_ago(10);
        let after = Local::now();
        
        assert!(result >= before - Duration::seconds(10));
        assert!(result <= after - Duration::seconds(10));
    }

    // Test the EasyTime::seconds_before (with base time)
    #[test]
    fn test_easy_time_seconds_before() {
        let date_time = Local::now();
        let result = EasyTime::<Local>::seconds_before(date_time, 10);
        let expected = date_time - Duration::seconds(10);
        assert_eq!(result, expected);
    }

    // Test the EasyTime::minutes_ago method
    #[test]
    fn test_easy_time_minutes_ago() {
        let before = Local::now();
        let result = EasyTime::<Local>::minutes_ago(10);
        let after = Local::now();
        
        assert!(result >= before - Duration::minutes(10));
        assert!(result <= after - Duration::minutes(10));
    }

    // Test the EasyTime::hours_ago method
    #[test]
    fn test_easy_time_hours_ago() {
        let before = Local::now();
        let result = EasyTime::<Local>::hours_ago(10);
        let after = Local::now();
        
        assert!(result >= before - Duration::hours(10));
        assert!(result <= after - Duration::hours(10));
    }

    // Test the EasyTime::days_ago method
    #[test]
    fn test_easy_time_days_ago() {
        let before = Local::now();
        let result = EasyTime::<Local>::days_ago(10);
        let after = Local::now();
        
        assert!(result >= before - Duration::days(10));
        assert!(result <= after - Duration::days(10));
    }

    #[test]
    fn test_months_from_now_crossing_boundary() {
        // Start at January 31st
        let date_time = Local.with_ymd_and_hms(2023, 1, 31, 12, 0, 0).unwrap();
        // Adding 1 month from January 31 should yield February 28 (non-leap year)
        let result = EasyTime::<Local>::months_from(date_time, 1);
        let expected = Local.with_ymd_and_hms(2023, 2, 28, 12, 0, 0).unwrap();
        assert_eq!(result, expected);
    }

    // Test months_from_now during a leap year
    #[test]
    fn test_months_from_now_leap_year() {
        // Start at January 31st of a leap year (e.g., 2024)
        let date_time = Local.with_ymd_and_hms(2024, 1, 31, 12, 0, 0).unwrap();
        // Adding 1 month from January 31 on a leap year should yield February 29, 2024
        let result = EasyTime::<Local>::months_from(date_time, 1);
        let expected = Local.with_ymd_and_hms(2024, 2, 29, 12, 0, 0).unwrap();
        assert_eq!(result, expected);
    }

    // Test months_ago crossing a year boundary
    #[test]
    fn test_months_ago_crossing_year_boundary() {
        // Start at March 15, 2023
        let date_time = Local.with_ymd_and_hms(2023, 3, 15, 10, 0, 0).unwrap();
        // Subtracting 4 months from March 15, 2023, should give November 15, 2022
        let result = EasyTime::<Local>::months_before(date_time, 4);
        let expected = Local.with_ymd_and_hms(2022, 11, 15, 10, 0, 0).unwrap();
        assert_eq!(result, expected);
    }

    // Test years_from_now with a leap year transition
    #[test]
    fn test_years_from_now_leap_transition() {
        // Start at Feb 29, 2024 (a leap year day)
        let date_time = Local.with_ymd_and_hms(2024, 2, 29, 12, 0, 0).unwrap();
        // Adding 1 year to Feb 29, 2024 should result in Feb 28, 2025 (Feb 29 doesn't exist in 2025)
        let result = EasyTime::<Local>::years_from(date_time, 1);
        let expected = Local.with_ymd_and_hms(2025, 2, 28, 12, 0, 0).unwrap();
        assert_eq!(result, expected);
    }

    // Test years_ago beyond a leap year
    #[test]
    fn test_years_ago_leap_transition() {
        // Start at Feb 29, 2024
        let date_time = Local.with_ymd_and_hms(2024, 2, 29, 12, 0, 0).unwrap();
        // Going 1 year ago from Feb 29, 2024 would be Feb 28, 2023 (since Feb 29 doesn't exist in 2023)
        let result = EasyTime::<Local>::years_before(date_time, 1);
        let expected = Local.with_ymd_and_hms(2023, 2, 28, 12, 0, 0).unwrap();
        assert_eq!(result, expected);
    }

    // Test larger offsets: decades_from_now
    #[test]
    fn test_decades_from_now() {
        let date_time = Local.with_ymd_and_hms(2023, 7, 15, 8, 30, 0).unwrap();
        // Adding 2 decades = 20 years
        let result = EasyTime::<Local>::decades_from(date_time, 2);
        let expected_year = 2023 + 20;
        let expected = Local
            .with_ymd_and_hms(expected_year, 7, 15, 8, 30, 0)
            .unwrap();
        assert_eq!(result, expected);
    }

    // Test centuries_from_now
    #[test]
    fn test_centuries_from_now() {
        let date_time = Local.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
        // Adding 1 century = 100 years
        let result = EasyTime::<Local>::centuries_from(date_time, 1);
        let expected_year = 2100;
        let expected = Local
            .with_ymd_and_hms(expected_year, 1, 1, 0, 0, 0)
            .unwrap();
        assert_eq!(result, expected);
    }

    // Test millenniums_from_now
    #[test]
    fn test_millenniums_from_now() {
        let date_time = Local.with_ymd_and_hms(1000, 6, 1, 12, 0, 0).unwrap();
        // Adding 1 millennium = 1000 years
        let result = EasyTime::<Local>::millenniums_from(date_time, 1);
        let expected_year = 2000;
        let expected = Local
            .with_ymd_and_hms(expected_year, 6, 1, 12, 0, 0)
            .unwrap();
        assert_eq!(result, expected);
    }

    // Test UTC methods
    #[test]
    fn test_easy_time_utc_days_from_now() {
        let before = Utc::now();
        let result = EasyTime::<Utc>::days_from_now(10);
        let after = Utc::now();
        
        assert!(result >= before + Duration::days(10));
        assert!(result <= after + Duration::days(10));
    }

    // Test with large negative years (checking behavior far in the past)
    #[test]
    fn test_years_ago_large_negative() {
        let date_time = Local.with_ymd_and_hms(2023, 3, 1, 0, 0, 0).unwrap();
        // Going 100 years ago from 2023-03-01 => 1923-03-01
        let result = EasyTime::<Local>::years_before(date_time, 100);
        let expected = Local.with_ymd_and_hms(1923, 3, 1, 0, 0, 0).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_decades_ago() {
        let date_time = Local.with_ymd_and_hms(2023, 7, 15, 8, 30, 0).unwrap();
        // Subtracting 2 decades => 20 years
        let result = EasyTime::<Local>::decades_before(date_time, 2);
        let expected_year = 2023 - 20; // 2003
        let expected = Local
            .with_ymd_and_hms(expected_year, 7, 15, 8, 30, 0)
            .unwrap();
        assert_eq!(result, expected);
    }

    // Test centuries_ago
    #[test]
    fn test_centuries_ago() {
        let date_time = Local.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();
        // Subtracting 1 century => -100 years = 1900
        let result = EasyTime::<Local>::centuries_before(date_time, 1);
        let expected = Local.with_ymd_and_hms(1900, 1, 1, 0, 0, 0).unwrap();
        assert_eq!(result, expected);
    }

    // Test millenniums_ago
    #[test]
    fn test_millenniums_ago() {
        let date_time = Local.with_ymd_and_hms(2000, 6, 1, 12, 0, 0).unwrap();
        // Subtracting 1 millennium => -1000 years = year 1000
        let result = EasyTime::<Local>::millenniums_before(date_time, 1);
        let expected = Local.with_ymd_and_hms(1000, 6, 1, 12, 0, 0).unwrap();
        assert_eq!(result, expected);
    }

    // Test format_datetime
    #[test]
    fn test_format_datetime() {
        let date_time = Local.with_ymd_and_hms(2023, 10, 1, 12, 34, 56).unwrap();
        let formatted = easy_time::format_datetime(&date_time);
        assert_eq!(formatted, "2023-10-01 12:34:56");
    }

    // Test format_datetime_with
    #[test]
    fn test_format_datetime_with() {
        let date_time = Local.with_ymd_and_hms(2023, 10, 1, 12, 34, 56).unwrap();
        let formatted = easy_time::format_datetime_with(&date_time, "%Y/%m/%d-%H:%M");
        assert_eq!(formatted, "2023/10/01-12:34");
    }

    // Test format_datetime_with_timezone
    #[test]
    fn test_format_datetime_with_timezone() {
        let date_time = Local.with_ymd_and_hms(2024, 3, 5, 8, 9, 7).unwrap();
        let result = easy_time::format_datetime_with_timezone(&date_time);
        assert!(
            result.starts_with("2024-03-05 08:09:07 "),
            "Result was: {}",
            result
        );
        assert!(
            result.contains(" +") || result.contains(" -"),
            "Timezone offset not found in string: {}",
            result
        );
    }

    // Test format_datetime_with_timezone_format
    #[test]
    fn test_format_datetime_with_timezone_format() {
        let date_time = Local.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let custom_format = "%m-%d-%Y %H:%M:%S";
        let result = easy_time::format_datetime_with_timezone_format(&date_time, custom_format);
        assert!(result.contains("01-01-2024 00:00:00"), "Got: {}", result);
        assert!(
            result.contains(" +") || result.contains(" -"),
            "Timezone offset not found in string: {}",
            result
        );
    }

    // Test to_timestamp
    #[test]
    fn test_to_timestamp() {
        let date_time = Local.with_ymd_and_hms(2023, 5, 1, 12, 34, 56).unwrap();
        let expected = date_time.timestamp();
        assert_eq!(easy_time::to_timestamp(&date_time), expected);
    }

    // Test to_date
    #[test]
    fn test_to_date() {
        let date_time = Local.with_ymd_and_hms(2023, 11, 15, 10, 11, 12).unwrap();
        assert_eq!(easy_time::to_date(&date_time), "2023-11-15");
    }

    // Test to_time
    #[test]
    fn test_to_time() {
        let date_time = Local.with_ymd_and_hms(2023, 11, 15, 10, 11, 12).unwrap();
        assert_eq!(easy_time::to_time(&date_time), "10:11:12");
    }

    // Test is_leap_year
    #[test]
    fn test_is_leap_year() {
        assert!(EasyTime::<Local>::is_leap_year(2024));
        assert!(!EasyTime::<Local>::is_leap_year(2023));
        assert!(EasyTime::<Local>::is_leap_year(2000));
        assert!(!EasyTime::<Local>::is_leap_year(1900));
    }

    // Test days_in_month
    #[test]
    fn test_days_in_month() {
        assert_eq!(EasyTime::<Local>::days_in_month(2024, 2), 29); // Leap year
        assert_eq!(EasyTime::<Local>::days_in_month(2023, 2), 28); // Non-leap year
        assert_eq!(EasyTime::<Local>::days_in_month(2023, 1), 31);
        assert_eq!(EasyTime::<Local>::days_in_month(2023, 4), 30);
    }

    // Test convenience UTC methods
    #[test]
    fn test_utc_convenience_methods() {
        let before = Utc::now();
        let result = EasyTime::<Local>::utc_days_from_now(10);
        let after = Utc::now();
        
        assert!(result >= before + Duration::days(10));
        assert!(result <= after + Duration::days(10));
    }

    // Test weeks_from_now
    #[test]
    fn test_weeks_from_now() {
        let before = Local::now();
        let result = EasyTime::<Local>::weeks_from_now(2);
        let after = Local::now();
        
        assert!(result >= before + Duration::weeks(2));
        assert!(result <= after + Duration::weeks(2));
    }

    // Test weeks_ago
    #[test]
    fn test_weeks_ago() {
        let before = Local::now();
        let result = EasyTime::<Local>::weeks_ago(2);
        let after = Local::now();
        
        assert!(result >= before - Duration::weeks(2));
        assert!(result <= after - Duration::weeks(2));
    }
}
