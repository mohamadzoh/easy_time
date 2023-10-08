extern crate easy_time;
use chrono::prelude::Local;
use chrono::Duration;
use easy_time::EasyTime;


#[cfg(test)]
mod tests {
    use super::*;

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

}
