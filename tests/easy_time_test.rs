use chrono::{DateTime, FixedOffset, Local, TimeDelta, TimeZone, Timelike, Utc};
use chrono_tz::America::New_York;
use chrono_tz::Asia::Beirut;
use chrono_tz::Tz;
use easy_time::{
    EasyTime, Unit, days_in_month, format_datetime, format_datetime_with,
    format_datetime_with_timezone, humanize, humanize_delta, is_leap_year, to_date, to_time,
};

fn utc(y: i32, mo: u32, d: u32, h: u32, mi: u32, s: u32) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(y, mo, d, h, mi, s).unwrap()
}

fn ny(y: i32, mo: u32, d: u32, h: u32, mi: u32, s: u32) -> DateTime<Tz> {
    New_York
        .with_ymd_and_hms(y, mo, d, h, mi, s)
        .single()
        .unwrap()
}

fn last_nano(time: DateTime<Utc>) -> DateTime<Utc> {
    time.with_nanosecond(999_999_999).unwrap()
}

const ALL_UNITS: [Unit; 10] = [
    Unit::Seconds,
    Unit::Minutes,
    Unit::Hours,
    Unit::Days,
    Unit::Weeks,
    Unit::Months,
    Unit::Years,
    Unit::Decades,
    Unit::Centuries,
    Unit::Millenniums,
];

// ---------------------------------------------------------------- relative to now

#[test]
fn now_methods_are_relative_to_now() {
    let before = Utc::now();
    let future = EasyTime::<Utc>::days_from_now(10);
    let past = EasyTime::<Utc>::weeks_ago(2);
    let after = Utc::now();
    assert!(future >= before + TimeDelta::days(10) && future <= after + TimeDelta::days(10));
    assert!(past >= before - TimeDelta::weeks(2) && past <= after - TimeDelta::weeks(2));

    let before = Local::now();
    let future = EasyTime::<Local>::seconds_from_now(30);
    let past = EasyTime::<Local>::hours_ago(3);
    let after = Local::now();
    assert!(future >= before + TimeDelta::seconds(30) && future <= after + TimeDelta::seconds(30));
    assert!(past >= before - TimeDelta::hours(3) && past <= after - TimeDelta::hours(3));
}

#[test]
fn every_now_method_exists_for_local_and_utc() {
    let _ = (
        EasyTime::<Local>::minutes_from_now(1),
        EasyTime::<Local>::months_ago(1),
        EasyTime::<Local>::years_from_now(1),
        EasyTime::<Local>::decades_ago(1),
        EasyTime::<Local>::centuries_from_now(1),
        EasyTime::<Local>::millenniums_ago(1),
        EasyTime::<Utc>::seconds_ago(1),
        EasyTime::<Utc>::minutes_ago(1),
        EasyTime::<Utc>::hours_from_now(1),
        EasyTime::<Utc>::days_ago(1),
        EasyTime::<Utc>::weeks_from_now(1),
        EasyTime::<Utc>::months_from_now(1),
        EasyTime::<Utc>::years_ago(1),
        EasyTime::<Utc>::decades_from_now(1),
        EasyTime::<Utc>::centuries_ago(1),
        EasyTime::<Utc>::millenniums_from_now(1),
    );
}

// ---------------------------------------------------------------- from a base time

#[test]
fn every_unit_moves_forward_and_back() {
    let base = utc(2024, 1, 31, 12, 0, 0);
    let cases = [
        (
            EasyTime::seconds_from(base, 90),
            utc(2024, 1, 31, 12, 1, 30),
        ),
        (
            EasyTime::seconds_before(base, 90),
            utc(2024, 1, 31, 11, 58, 30),
        ),
        (
            EasyTime::minutes_from(base, 90),
            utc(2024, 1, 31, 13, 30, 0),
        ),
        (
            EasyTime::minutes_before(base, 90),
            utc(2024, 1, 31, 10, 30, 0),
        ),
        (EasyTime::hours_from(base, 25), utc(2024, 2, 1, 13, 0, 0)),
        (EasyTime::hours_before(base, 25), utc(2024, 1, 30, 11, 0, 0)),
        (EasyTime::days_from(base, 1), utc(2024, 2, 1, 12, 0, 0)),
        (EasyTime::days_before(base, 31), utc(2023, 12, 31, 12, 0, 0)),
        (EasyTime::weeks_from(base, 1), utc(2024, 2, 7, 12, 0, 0)),
        (EasyTime::weeks_before(base, 1), utc(2024, 1, 24, 12, 0, 0)),
        (EasyTime::months_from(base, 1), utc(2024, 2, 29, 12, 0, 0)),
        (
            EasyTime::months_before(base, 2),
            utc(2023, 11, 30, 12, 0, 0),
        ),
        (EasyTime::years_from(base, 1), utc(2025, 1, 31, 12, 0, 0)),
        (
            EasyTime::years_before(base, 100),
            utc(1924, 1, 31, 12, 0, 0),
        ),
        (EasyTime::decades_from(base, 2), utc(2044, 1, 31, 12, 0, 0)),
        (
            EasyTime::decades_before(base, 2),
            utc(2004, 1, 31, 12, 0, 0),
        ),
        (
            EasyTime::centuries_from(base, 1),
            utc(2124, 1, 31, 12, 0, 0),
        ),
        (
            EasyTime::centuries_before(base, 1),
            utc(1924, 1, 31, 12, 0, 0),
        ),
        (
            EasyTime::millenniums_from(base, 1),
            utc(3024, 1, 31, 12, 0, 0),
        ),
        (
            EasyTime::millenniums_before(base, 1),
            utc(1024, 1, 31, 12, 0, 0),
        ),
    ];
    for (i, (got, expected)) in cases.into_iter().enumerate() {
        assert_eq!(got, expected, "case {i}");
    }
}

#[test]
fn negative_amounts_move_the_other_way() {
    let base = utc(2024, 3, 15, 10, 0, 0);
    assert_eq!(
        EasyTime::days_from(base, -3),
        EasyTime::days_before(base, 3)
    );
    assert_eq!(
        EasyTime::months_before(base, -4),
        EasyTime::months_from(base, 4)
    );
    assert_eq!(
        EasyTime::<Utc>::checked_add(base, 0, Unit::Years),
        Some(base)
    );
}

#[test]
fn months_and_years_clamp_to_month_end() {
    assert_eq!(
        EasyTime::months_from(utc(2023, 1, 31, 12, 0, 0), 1),
        utc(2023, 2, 28, 12, 0, 0)
    );
    assert_eq!(
        EasyTime::months_from(utc(2024, 1, 31, 12, 0, 0), 1),
        utc(2024, 2, 29, 12, 0, 0)
    );
    assert_eq!(
        EasyTime::months_before(utc(2023, 3, 15, 10, 0, 0), 4),
        utc(2022, 11, 15, 10, 0, 0)
    );

    let leap_day = utc(2024, 2, 29, 12, 0, 0);
    assert_eq!(
        EasyTime::years_from(leap_day, 1),
        utc(2025, 2, 28, 12, 0, 0)
    );
    assert_eq!(
        EasyTime::years_before(leap_day, 1),
        utc(2023, 2, 28, 12, 0, 0)
    );
    assert_eq!(
        EasyTime::years_from(leap_day, 4),
        utc(2028, 2, 29, 12, 0, 0)
    );
}

#[test]
fn works_in_any_time_zone_without_turbofish() {
    let offset = FixedOffset::east_opt(5 * 3600 + 1800).unwrap();
    let base = offset.with_ymd_and_hms(2024, 1, 31, 23, 0, 0).unwrap();
    assert_eq!(
        EasyTime::months_from(base, 1),
        offset.with_ymd_and_hms(2024, 2, 29, 23, 0, 0).unwrap()
    );
    assert_eq!(
        EasyTime::years_before(ny(2024, 7, 1, 9, 0, 0), 1),
        ny(2023, 7, 1, 9, 0, 0)
    );
}

// ---------------------------------------------------------------- daylight saving time

#[test]
fn days_keep_wall_clock_time_across_dst_but_hours_are_exact() {
    let before_spring_forward = ny(2023, 3, 11, 12, 0, 0);
    let next_day = EasyTime::days_from(before_spring_forward, 1);
    assert_eq!(next_day, ny(2023, 3, 12, 12, 0, 0));
    assert_eq!(next_day - before_spring_forward, TimeDelta::hours(23));
    assert_eq!(
        EasyTime::hours_from(before_spring_forward, 24),
        ny(2023, 3, 12, 13, 0, 0)
    );
    assert_eq!(
        EasyTime::weeks_before(ny(2023, 3, 15, 12, 0, 0), 1),
        ny(2023, 3, 8, 12, 0, 0)
    );
}

#[test]
fn landing_in_a_dst_gap_moves_forward_instead_of_panicking() {
    // 02:30 on 2023-03-12 does not exist in New York; clocks jump from 02:00 to 03:00.
    assert_eq!(
        EasyTime::months_from(ny(2023, 2, 12, 2, 30, 0), 1),
        ny(2023, 3, 12, 3, 30, 0)
    );
    assert_eq!(
        EasyTime::days_from(ny(2023, 3, 11, 2, 30, 0), 1),
        ny(2023, 3, 12, 3, 30, 0)
    );
    // Beirut skips midnight: 2024-03-31 00:30 does not exist.
    let base = Beirut
        .with_ymd_and_hms(2024, 1, 31, 0, 30, 0)
        .single()
        .unwrap();
    let moved = EasyTime::months_from(base, 2);
    assert_eq!(
        moved,
        Beirut
            .with_ymd_and_hms(2024, 3, 31, 1, 30, 0)
            .single()
            .unwrap()
    );
}

#[test]
fn landing_in_a_dst_overlap_keeps_the_offset_when_it_can() {
    // 01:30 on 2023-11-05 happens twice in New York.
    let first = New_York
        .with_ymd_and_hms(2023, 11, 5, 1, 30, 0)
        .earliest()
        .unwrap();
    let second = New_York
        .with_ymd_and_hms(2023, 11, 5, 1, 30, 0)
        .latest()
        .unwrap();
    assert_eq!(EasyTime::hours_from(first, 1), second);
    assert_eq!(EasyTime::days_from(second, 0), second);
    assert_eq!(EasyTime::days_from(ny(2023, 11, 4, 1, 30, 0), 1), first);
    assert_eq!(
        EasyTime::start_of(second, Unit::Hours),
        New_York
            .with_ymd_and_hms(2023, 11, 5, 1, 0, 0)
            .latest()
            .unwrap()
    );
}

// ---------------------------------------------------------------- overflow

#[test]
fn checked_add_returns_none_when_out_of_range() {
    let base = utc(2024, 1, 31, 12, 0, 0);
    for unit in ALL_UNITS {
        assert_eq!(
            EasyTime::checked_add(base, i64::MAX, unit),
            None,
            "{unit:?}"
        );
        assert_eq!(
            EasyTime::checked_add(base, i64::MIN, unit),
            None,
            "{unit:?}"
        );
    }
    // Used to be truncated to i32 and silently return one month later.
    assert_eq!(
        EasyTime::checked_add(base, 4_294_967_297, Unit::Months),
        None
    );
    assert_eq!(
        EasyTime::checked_add(base, 3_000_000, Unit::Millenniums),
        None
    );
}

#[test]
#[should_panic(expected = "out of range")]
fn named_methods_panic_with_a_clear_message_when_out_of_range() {
    let _ = EasyTime::days_from(utc(2024, 1, 1, 0, 0, 0), i64::MAX);
}

#[test]
#[should_panic(expected = "out of range")]
fn before_with_i64_min_panics_instead_of_overflowing() {
    let _ = EasyTime::years_before(utc(2024, 1, 1, 0, 0, 0), i64::MIN);
}

// ---------------------------------------------------------------- start_of / end_of

#[test]
fn start_and_end_of_every_unit() {
    let time = utc(2024, 5, 16, 15, 42, 7)
        .with_nanosecond(123_456_789)
        .unwrap(); // a Thursday
    let cases = [
        (
            Unit::Seconds,
            utc(2024, 5, 16, 15, 42, 7),
            utc(2024, 5, 16, 15, 42, 7),
        ),
        (
            Unit::Minutes,
            utc(2024, 5, 16, 15, 42, 0),
            utc(2024, 5, 16, 15, 42, 59),
        ),
        (
            Unit::Hours,
            utc(2024, 5, 16, 15, 0, 0),
            utc(2024, 5, 16, 15, 59, 59),
        ),
        (
            Unit::Days,
            utc(2024, 5, 16, 0, 0, 0),
            utc(2024, 5, 16, 23, 59, 59),
        ),
        (
            Unit::Weeks,
            utc(2024, 5, 13, 0, 0, 0),
            utc(2024, 5, 19, 23, 59, 59),
        ),
        (
            Unit::Months,
            utc(2024, 5, 1, 0, 0, 0),
            utc(2024, 5, 31, 23, 59, 59),
        ),
        (
            Unit::Years,
            utc(2024, 1, 1, 0, 0, 0),
            utc(2024, 12, 31, 23, 59, 59),
        ),
        (
            Unit::Decades,
            utc(2020, 1, 1, 0, 0, 0),
            utc(2029, 12, 31, 23, 59, 59),
        ),
        (
            Unit::Centuries,
            utc(2000, 1, 1, 0, 0, 0),
            utc(2099, 12, 31, 23, 59, 59),
        ),
        (
            Unit::Millenniums,
            utc(2000, 1, 1, 0, 0, 0),
            utc(2999, 12, 31, 23, 59, 59),
        ),
    ];
    for (unit, start, end) in cases {
        assert_eq!(EasyTime::start_of(time, unit), start, "start of {unit:?}");
        assert_eq!(
            EasyTime::end_of(time, unit),
            last_nano(end),
            "end of {unit:?}"
        );
    }
}

#[test]
fn start_and_end_of_edge_cases() {
    // A Sunday belongs to the week that started the Monday before.
    assert_eq!(
        EasyTime::start_of(utc(2024, 5, 19, 8, 0, 0), Unit::Weeks),
        utc(2024, 5, 13, 0, 0, 0)
    );
    // The end of a month starting from the 31st does not skip February.
    assert_eq!(
        EasyTime::end_of(utc(2024, 1, 31, 8, 0, 0), Unit::Months),
        last_nano(utc(2024, 1, 31, 23, 59, 59))
    );
    // Negative years round down.
    assert_eq!(
        EasyTime::start_of(utc(-5, 6, 1, 0, 0, 0), Unit::Decades),
        utc(-10, 1, 1, 0, 0, 0)
    );
}

#[test]
fn start_and_end_of_day_across_dst() {
    // Beirut's day of 2024-03-31 starts at 01:00 because midnight is skipped.
    let start = EasyTime::start_of(
        Beirut.with_ymd_and_hms(2024, 3, 31, 12, 0, 0).unwrap(),
        Unit::Days,
    );
    assert_eq!(
        start,
        Beirut.with_ymd_and_hms(2024, 3, 31, 1, 0, 0).unwrap()
    );

    // New York's 2023-11-05 lasts 25 hours.
    let noon = ny(2023, 11, 5, 12, 0, 0);
    let (start, end) = (
        EasyTime::start_of(noon, Unit::Days),
        EasyTime::end_of(noon, Unit::Days),
    );
    assert_eq!(start, ny(2023, 11, 5, 0, 0, 0));
    assert_eq!(
        end - start,
        TimeDelta::hours(25) - TimeDelta::nanoseconds(1)
    );
}

#[test]
fn start_of_a_period_inside_a_long_dst_gap_is_when_the_gap_ends() {
    // Casey's clocks jumped from 00:01 to 03:01 on 2021-10-03, so the hour "03" began at 03:01.
    let casey = chrono_tz::Antarctica::Casey;
    let time = casey.with_ymd_and_hms(2021, 10, 3, 3, 4, 0).unwrap();
    assert_eq!(
        EasyTime::start_of(time, Unit::Hours),
        casey.with_ymd_and_hms(2021, 10, 3, 3, 1, 0).unwrap()
    );
    // The hour "00" before the jump lasted one minute; it still counts as the next hour after 23:xx.
    let before = casey.with_ymd_and_hms(2021, 10, 2, 23, 7, 0).unwrap();
    let end = EasyTime::end_of(before, Unit::Hours);
    assert_eq!(
        end + TimeDelta::nanoseconds(1),
        casey.with_ymd_and_hms(2021, 10, 3, 0, 0, 0).unwrap()
    );
}

#[test]
fn a_day_with_a_repeated_midnight_starts_at_the_first_one() {
    // Havana turns 01:00 back to 00:00 on 2023-11-05, so midnight happens twice.
    let havana = chrono_tz::America::Havana;
    let noon = havana.with_ymd_and_hms(2023, 11, 5, 12, 0, 0).unwrap();
    let first_midnight = havana
        .with_ymd_and_hms(2023, 11, 5, 0, 0, 0)
        .earliest()
        .unwrap();
    assert_eq!(EasyTime::start_of(noon, Unit::Days), first_midnight);
    let day_before = havana.with_ymd_and_hms(2023, 11, 4, 12, 0, 0).unwrap();
    assert_eq!(
        EasyTime::end_of(day_before, Unit::Days) + TimeDelta::nanoseconds(1),
        first_midnight
    );
}

#[test]
fn an_hour_cut_short_by_an_off_hour_dst_change_ends_when_the_clock_turns_back() {
    // Chatham turns 03:45 back to 02:45 on 2021-04-04.
    let chatham = chrono_tz::Pacific::Chatham;
    let time = chatham
        .with_ymd_and_hms(2021, 4, 4, 3, 42, 0)
        .earliest()
        .unwrap();
    let turn_back = chatham
        .with_ymd_and_hms(2021, 4, 4, 2, 45, 0)
        .latest()
        .unwrap();
    assert_eq!(
        EasyTime::end_of(time, Unit::Hours) + TimeDelta::nanoseconds(1),
        turn_back
    );
}

#[test]
fn start_of_never_comes_after_end_of() {
    let time = ny(2024, 2, 29, 23, 59, 59);
    for unit in ALL_UNITS {
        let (start, end) = (EasyTime::start_of(time, unit), EasyTime::end_of(time, unit));
        assert!(
            start <= time && time <= end,
            "{unit:?}: {start} <= {time} <= {end}"
        );
    }
}

// ---------------------------------------------------------------- humanize

#[test]
fn humanize_delta_uses_the_largest_whole_unit() {
    let cases = [
        (TimeDelta::zero(), "just now"),
        (TimeDelta::milliseconds(400), "just now"),
        (TimeDelta::milliseconds(600), "in 1 second"),
        (TimeDelta::seconds(-1), "1 second ago"),
        (TimeDelta::seconds(59), "in 59 seconds"),
        (TimeDelta::minutes(90), "in 1 hour"),
        (TimeDelta::days(-3), "3 days ago"),
        (
            TimeDelta::days(14) - TimeDelta::milliseconds(1),
            "in 2 weeks",
        ),
        (TimeDelta::days(45), "in 1 month"),
        (TimeDelta::days(-800), "2 years ago"),
    ];
    for (delta, expected) in cases {
        assert_eq!(humanize_delta(delta), expected, "{delta}");
    }
    // Extremes don't overflow.
    assert!(humanize_delta(TimeDelta::MAX).starts_with("in "));
    assert!(humanize_delta(TimeDelta::MIN).ends_with(" ago"));
}

#[test]
fn humanize_is_relative_to_now() {
    assert_eq!(humanize(&EasyTime::<Utc>::days_from_now(3)), "in 3 days");
    assert_eq!(humanize(&EasyTime::<Local>::hours_ago(5)), "5 hours ago");
}

// ---------------------------------------------------------------- formatting

#[test]
fn formatting() {
    let time = utc(2023, 10, 1, 12, 34, 56);
    assert_eq!(format_datetime(&time), "2023-10-01 12:34:56");
    assert_eq!(
        format_datetime_with(&time, "%Y/%m/%d-%H:%M").as_deref(),
        Some("2023/10/01-12:34")
    );
    assert_eq!(to_date(&time), "2023-10-01");
    assert_eq!(to_time(&time), "12:34:56");
}

#[test]
fn invalid_format_string_returns_none_instead_of_panicking() {
    assert_eq!(format_datetime_with(&utc(2023, 10, 1, 0, 0, 0), "%Q"), None);
    assert_eq!(format_datetime_with(&utc(2023, 10, 1, 0, 0, 0), "%"), None);
}

#[test]
fn format_with_timezone_always_shows_a_numeric_offset() {
    assert_eq!(
        format_datetime_with_timezone(&utc(2024, 3, 5, 8, 9, 7)),
        "2024-03-05 08:09:07 +00:00"
    );
    assert_eq!(
        format_datetime_with_timezone(&ny(2024, 1, 1, 0, 0, 0)),
        "2024-01-01 00:00:00 -05:00"
    );
    let india = FixedOffset::east_opt(5 * 3600 + 1800).unwrap();
    let time = india.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    assert_eq!(
        format_datetime_with_timezone(&time),
        "2024-01-01 00:00:00 +05:30"
    );
}

// ---------------------------------------------------------------- calendar helpers

#[test]
fn leap_years() {
    assert!(is_leap_year(2024));
    assert!(is_leap_year(2000));
    assert!(!is_leap_year(2023));
    assert!(!is_leap_year(1900));
    assert!(is_leap_year(-4));
}

#[test]
fn days_in_each_month() {
    let lengths = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    for (month, days) in (1..=12).zip(lengths) {
        assert_eq!(days_in_month(2023, month), Some(days), "month {month}");
    }
    assert_eq!(days_in_month(2024, 2), Some(29));
    assert_eq!(days_in_month(2024, 0), None);
    assert_eq!(days_in_month(2024, 13), None);
}
