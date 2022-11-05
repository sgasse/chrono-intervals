use chrono::{DateTime, Datelike, NaiveTime, TimeZone, Utc};
use chrono_intervals::{Error, Grouping, IntervalGenerator};

#[test]
fn test_per_week_regular() -> Result<(), Error> {
    let begin = DateTime::parse_from_rfc3339("2022-10-04T08:23:45.000000Z")?;
    let end = DateTime::parse_from_rfc3339("2022-10-18T08:23:45.000000Z")?;

    let weekly_intervals = IntervalGenerator::new()
        .with_grouping(Grouping::PerWeek)
        .get_intervals(begin, end);
    let expected_intervals = vec![
        (
            Utc.ymd(2022, 10, 3).and_hms(0, 0, 0),
            Utc.ymd(2022, 10, 9).and_hms_milli(23, 59, 59, 999),
        ),
        (
            Utc.ymd(2022, 10, 10).and_hms(0, 0, 0),
            Utc.ymd(2022, 10, 16).and_hms_milli(23, 59, 59, 999),
        ),
        (
            Utc.ymd(2022, 10, 17).and_hms(0, 0, 0),
            Utc.ymd(2022, 10, 23).and_hms_milli(23, 59, 59, 999),
        ),
    ];
    assert_eq!(weekly_intervals, expected_intervals);

    Ok(())
}

#[test]
fn test_per_week_over_several_months() -> Result<(), Error> {
    let begin = DateTime::parse_from_rfc3339("2022-09-09T08:23:45.000000Z")?;
    let end = DateTime::parse_from_rfc3339("2022-11-09T08:23:45.000000Z")?;

    let weekly_intervals = IntervalGenerator::new()
        .with_grouping(Grouping::PerWeek)
        .get_intervals(begin, end);
    assert_eq!(weekly_intervals.len(), 10);
    for interval in weekly_intervals {
        assert_eq!(interval.0.weekday(), chrono::Weekday::Mon);
        assert_eq!(interval.1.weekday(), chrono::Weekday::Sun);
        assert_eq!(interval.0.time(), NaiveTime::from_hms(0, 0, 0));
        assert_eq!(
            interval.1.time(),
            NaiveTime::from_hms_milli(23, 59, 59, 999)
        );
    }

    Ok(())
}

#[test]
fn test_per_week_over_a_year() -> Result<(), Error> {
    let begin = DateTime::parse_from_rfc3339("2021-09-09T08:23:45.000000Z")?;
    let end = DateTime::parse_from_rfc3339("2022-09-08T08:23:45.000000Z")?;

    let weekly_intervals = IntervalGenerator::new()
        .with_grouping(Grouping::PerWeek)
        .get_intervals(begin, end);
    assert_eq!(weekly_intervals.len(), 53);
    for interval in weekly_intervals.iter() {
        assert_eq!(interval.0.weekday(), chrono::Weekday::Mon);
        assert_eq!(interval.1.weekday(), chrono::Weekday::Sun);
        assert_eq!(interval.0.time(), NaiveTime::from_hms(0, 0, 0));
        assert_eq!(
            interval.1.time(),
            NaiveTime::from_hms_milli(23, 59, 59, 999)
        );
    }
    assert_eq!(weekly_intervals.first().unwrap().0.year(), 2021);
    assert_eq!(weekly_intervals.last().unwrap().0.year(), 2022);

    Ok(())
}
