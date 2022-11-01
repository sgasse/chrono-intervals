use chrono::{DateTime, Datelike, NaiveTime, TimeZone, Utc};
use chrono_intervals::{get_extended_utc_intervals, Error, Grouping};

#[test]
fn test_per_day_regular() -> Result<(), Error> {
    let begin = DateTime::parse_from_rfc3339("2022-06-25T08:23:45.000000Z")?;
    let end = DateTime::parse_from_rfc3339("2022-06-27T08:23:45.000000Z")?;

    let daily_intervals = get_extended_utc_intervals(begin, end, &Grouping::PerDay, 0);
    assert_eq!(
        daily_intervals,
        vec![
            (
                Utc.ymd(2022, 6, 25).and_hms(0, 0, 0),
                Utc.ymd(2022, 6, 25).and_hms_milli(23, 59, 59, 999),
            ),
            (
                Utc.ymd(2022, 6, 26).and_hms(0, 0, 0),
                Utc.ymd(2022, 6, 26).and_hms_milli(23, 59, 59, 999),
            ),
            (
                Utc.ymd(2022, 6, 27).and_hms(0, 0, 0),
                Utc.ymd(2022, 6, 27).and_hms_milli(23, 59, 59, 999),
            ),
        ]
    );

    Ok(())
}

#[test]
fn test_per_day_over_a_month() -> Result<(), Error> {
    let begin = DateTime::parse_from_rfc3339("2022-06-25T08:23:45.000000Z")?;
    let end = DateTime::parse_from_rfc3339("2022-07-25T08:23:45.000000Z")?;

    let daily_intervals = get_extended_utc_intervals(begin, end, &Grouping::PerDay, 0);
    assert_eq!(daily_intervals.len(), 31);
    for interval in daily_intervals {
        assert_eq!(interval.0.day(), interval.1.day());
        assert_eq!(interval.0.time(), NaiveTime::from_hms(0, 0, 0));
        assert_eq!(
            interval.1.time(),
            NaiveTime::from_hms_milli(23, 59, 59, 999)
        );
    }

    Ok(())
}

#[test]
fn test_per_day_over_a_year() -> Result<(), Error> {
    let begin = DateTime::parse_from_rfc3339("2021-06-25T08:23:45.000000Z")?;
    let end = DateTime::parse_from_rfc3339("2022-06-24T08:23:45.000000Z")?;

    let daily_intervals = get_extended_utc_intervals(begin, end, &Grouping::PerDay, 0);
    assert_eq!(daily_intervals.len(), 365);
    for interval in daily_intervals.iter() {
        assert_eq!(interval.0.day(), interval.1.day());
        assert_eq!(interval.0.time(), NaiveTime::from_hms(0, 0, 0));
        assert_eq!(
            interval.1.time(),
            NaiveTime::from_hms_milli(23, 59, 59, 999)
        );
    }
    assert_eq!(daily_intervals.first().unwrap().0.year(), 2021);
    assert_eq!(daily_intervals.last().unwrap().0.year(), 2022);

    Ok(())
}
