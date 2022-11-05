use chrono::{DateTime, Datelike, Duration, NaiveTime, TimeZone, Utc};
use chrono_intervals::{Error, Grouping, IntervalGenerator};

#[test]
fn test_per_month_regular() -> Result<(), Error> {
    let begin = DateTime::parse_from_rfc3339("2022-06-04T08:23:45.000000Z")?;
    let end = DateTime::parse_from_rfc3339("2022-09-18T08:23:45.000000Z")?;

    let monthly_intervals = IntervalGenerator::new()
        .with_grouping(Grouping::PerMonth)
        .get_intervals(begin, end);
    let expected_intervals = vec![
        (
            Utc.ymd(2022, 6, 1).and_hms(0, 0, 0),
            Utc.ymd(2022, 6, 30).and_hms_milli(23, 59, 59, 999),
        ),
        (
            Utc.ymd(2022, 7, 1).and_hms(0, 0, 0),
            Utc.ymd(2022, 7, 31).and_hms_milli(23, 59, 59, 999),
        ),
        (
            Utc.ymd(2022, 8, 1).and_hms(0, 0, 0),
            Utc.ymd(2022, 8, 31).and_hms_milli(23, 59, 59, 999),
        ),
        (
            Utc.ymd(2022, 9, 1).and_hms(0, 0, 0),
            Utc.ymd(2022, 9, 30).and_hms_milli(23, 59, 59, 999),
        ),
    ];
    assert_eq!(monthly_intervals, expected_intervals);

    Ok(())
}

#[test]
fn test_per_month_over_several_years() -> Result<(), Error> {
    let begin = DateTime::parse_from_rfc3339("2020-09-09T08:23:45.000000Z")?;
    let end = DateTime::parse_from_rfc3339("2022-08-09T08:23:45.000000Z")?;

    let monthly_intervals = IntervalGenerator::new()
        .with_grouping(Grouping::PerMonth)
        .get_intervals(begin, end);
    assert_eq!(monthly_intervals.len(), 24);
    for interval in monthly_intervals.iter() {
        assert_eq!(interval.0.day(), 1);
        assert_eq!((interval.1 + Duration::milliseconds(1)).day(), 1);
        assert_eq!(interval.0.time(), NaiveTime::from_hms(0, 0, 0));
        assert_eq!(
            interval.1.time(),
            NaiveTime::from_hms_milli(23, 59, 59, 999)
        );
    }
    assert_eq!(monthly_intervals.first().unwrap().0.year(), 2020);
    assert_eq!(monthly_intervals.last().unwrap().0.year(), 2022);

    Ok(())
}
