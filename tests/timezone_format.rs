use chrono::{DateTime, TimeZone, Utc};
use chrono_intervals::{get_extended_utc_intervals_with_defaults, grouping::Grouping, Error};

#[test]
fn test_utc_begin_end_to_utc() -> Result<(), Error> {
    let begin = DateTime::parse_from_rfc3339("2022-09-29T08:23:45.000000Z")?;
    let end = DateTime::parse_from_rfc3339("2022-09-30T08:23:45.000000Z")?;

    let daily_intervals =
        get_extended_utc_intervals_with_defaults(begin, end, &Grouping::PerDay, 0);
    let expected_intervals = vec![
        (
            Utc.ymd(2022, 9, 29).and_hms(0, 0, 0),
            Utc.ymd(2022, 9, 29).and_hms_milli(23, 59, 59, 999),
        ),
        (
            Utc.ymd(2022, 9, 30).and_hms(0, 0, 0),
            Utc.ymd(2022, 9, 30).and_hms_milli(23, 59, 59, 999),
        ),
    ];
    assert_eq!(daily_intervals, expected_intervals);

    Ok(())
}

#[test]
fn test_cest_begin_end_to_utc() -> Result<(), Error> {
    // In Central European Summer Time (CEST), `begin` and `end` are on the 25th
    // and 26th. However due to the time difference with UTC, we will get the
    // UTC daily intervals for 24th and 25th.
    let begin = DateTime::parse_from_rfc3339("2022-09-25T01:23:45.000000+02:00")?;
    let end = DateTime::parse_from_rfc3339("2022-09-26T01:23:45.000000+02:00")?;

    let daily_intervals =
        get_extended_utc_intervals_with_defaults(begin, end, &Grouping::PerDay, 0);
    let expected_intervals = vec![
        (
            Utc.ymd(2022, 9, 24).and_hms(0, 0, 0),
            Utc.ymd(2022, 9, 24).and_hms_milli(23, 59, 59, 999),
        ),
        (
            Utc.ymd(2022, 9, 25).and_hms(0, 0, 0),
            Utc.ymd(2022, 9, 25).and_hms_milli(23, 59, 59, 999),
        ),
    ];
    assert_eq!(daily_intervals, expected_intervals);

    Ok(())
}

#[test]
fn test_pdt_begin_end_to_utc() -> Result<(), Error> {
    // In Pacific Daylight Time (PDT), `begin` and `end` are on the 23rd and
    // 24th. However due to the time difference with UTC, we will get the UTC
    // daily intervals for 24th and 25th.
    let begin = DateTime::parse_from_rfc3339("2022-09-23T22:23:45.000000-07:00")?;
    let end = DateTime::parse_from_rfc3339("2022-09-24T20:23:45.000000-07:00")?;

    let daily_intervals =
        get_extended_utc_intervals_with_defaults(begin, end, &Grouping::PerDay, 0);
    let expected_intervals = vec![
        (
            Utc.ymd(2022, 9, 24).and_hms(0, 0, 0),
            Utc.ymd(2022, 9, 24).and_hms_milli(23, 59, 59, 999),
        ),
        (
            Utc.ymd(2022, 9, 25).and_hms(0, 0, 0),
            Utc.ymd(2022, 9, 25).and_hms_milli(23, 59, 59, 999),
        ),
    ];
    assert_eq!(daily_intervals, expected_intervals);

    Ok(())
}
