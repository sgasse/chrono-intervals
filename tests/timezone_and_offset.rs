use chrono::{DateTime, TimeZone, Utc};
use chrono_intervals::{Error, Grouping, IntervalGenerator};

#[test]
fn test_utc_begin_end_to_utc() -> Result<(), Error> {
    let begin = DateTime::parse_from_rfc3339("2022-09-29T08:23:45.000000Z")?;
    let end = DateTime::parse_from_rfc3339("2022-09-30T08:23:45.000000Z")?;

    let daily_intervals = IntervalGenerator::new()
        .with_grouping(Grouping::PerDay)
        .get_intervals(begin, end);
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

    let daily_intervals = IntervalGenerator::new()
        .with_grouping(Grouping::PerDay)
        .get_intervals(begin, end);
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

    let daily_intervals = IntervalGenerator::new()
        .with_grouping(Grouping::PerDay)
        .get_intervals(begin, end);
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
fn test_cest_offset_days() -> Result<(), Error> {
    // We want to obtain daily intervals for days in CEST instead of in UTC.
    let begin = DateTime::parse_from_rfc3339("2022-09-25T10:23:45.000000+02:00")?;
    let end = DateTime::parse_from_rfc3339("2022-09-26T10:23:45.000000+02:00")?;

    // CEST is 2h ahead of UTC (towards the **east**), thus the
    // `offset_west_seconds` are -2*3600
    let cest_offset_west_seconds = -2 * 3600;

    let daily_intervals = IntervalGenerator::new()
        .with_grouping(Grouping::PerDay)
        .with_offset_west_secs(cest_offset_west_seconds)
        .get_intervals(begin, end);
    // In UTC, we expect the intervals to start 2h ahead of the day boundary.
    let expected_intervals = vec![
        (
            Utc.ymd(2022, 9, 24).and_hms(22, 0, 0),
            Utc.ymd(2022, 9, 25).and_hms_milli(21, 59, 59, 999),
        ),
        (
            Utc.ymd(2022, 9, 25).and_hms(22, 0, 0),
            Utc.ymd(2022, 9, 26).and_hms_milli(21, 59, 59, 999),
        ),
    ];
    assert_eq!(daily_intervals, expected_intervals);

    Ok(())
}

#[test]
fn test_pdt_offset_days() -> Result<(), Error> {
    // We want to obtain daily intervals for days in PDT instead of in UTC.
    let begin = DateTime::parse_from_rfc3339("2022-09-25T12:23:45.000000-07:00")?;
    let end = DateTime::parse_from_rfc3339("2022-09-26T12:23:45.000000-07:00")?;

    // PDT is 7h behind of UTC (towards the **west**), thus the
    // `offset_west_seconds` are 7*3600
    let pdt_offset_west_seconds = 7 * 3600;

    let daily_intervals = IntervalGenerator::new()
        .with_grouping(Grouping::PerDay)
        .with_offset_west_secs(pdt_offset_west_seconds)
        .get_intervals(begin, end);
    // In UTC, we expect the intervals to start 7h after the day boundary.
    let expected_intervals = vec![
        (
            Utc.ymd(2022, 9, 25).and_hms(7, 0, 0),
            Utc.ymd(2022, 9, 26).and_hms_milli(6, 59, 59, 999),
        ),
        (
            Utc.ymd(2022, 9, 26).and_hms(7, 0, 0),
            Utc.ymd(2022, 9, 27).and_hms_milli(6, 59, 59, 999),
        ),
    ];
    assert_eq!(daily_intervals, expected_intervals);

    Ok(())
}
