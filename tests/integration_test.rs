mod common;
use chrono::{DateTime, Duration, NaiveTime};
use chrono_intervals::{
    get_extended_utc_intervals_with_defaults, get_utc_intervals_opts, grouping::Grouping, Error,
};

#[test]
fn test_get_utc_intervals_precision() -> Result<(), Error> {
    let begin = DateTime::parse_from_rfc3339("2022-10-29T08:23:45.000000Z")?;
    let end = DateTime::parse_from_rfc3339("2022-11-01T08:23:45.000000Z")?;

    // Milli-seconds
    let with_milli_secs_precision = get_utc_intervals_opts(
        begin,
        end,
        &Grouping::PerDay,
        0,
        Duration::milliseconds(1),
        false,
        false,
    );

    for interval in with_milli_secs_precision {
        assert_eq!(interval.0.time(), NaiveTime::from_hms(0, 0, 0));
        assert_eq!(
            interval.1.time(),
            NaiveTime::from_hms_milli(23, 59, 59, 999)
        );
    }

    // Micro-seconds
    let with_micro_secs_precision = get_utc_intervals_opts(
        begin,
        end,
        &Grouping::PerDay,
        0,
        Duration::microseconds(1),
        false,
        false,
    );

    for interval in with_micro_secs_precision {
        assert_eq!(interval.0.time(), NaiveTime::from_hms(0, 0, 0));
        assert_eq!(
            interval.1.time(),
            NaiveTime::from_hms_micro(23, 59, 59, 999_999)
        );
    }

    // Nano-seconds
    let with_nano_secs_precision = get_utc_intervals_opts(
        begin,
        end,
        &Grouping::PerDay,
        0,
        Duration::nanoseconds(1),
        false,
        false,
    );

    for interval in with_nano_secs_precision {
        assert_eq!(interval.0.time(), NaiveTime::from_hms(0, 0, 0));
        assert_eq!(
            interval.1.time(),
            NaiveTime::from_hms_nano(23, 59, 59, 999_999_999)
        );
    }

    Ok(())
}

#[test]
fn test_get_utc_intervals_extended_begin_end() -> Result<(), Error> {
    let begin = DateTime::parse_from_rfc3339("2022-10-29T08:23:45.000000Z")?;
    let end = DateTime::parse_from_rfc3339("2022-11-01T08:23:45.000000Z")?;

    // Check that we get grouped intervals enclosed by `begin` and `end`
    let non_extended_intervals = get_utc_intervals_opts(
        begin,
        end,
        &Grouping::PerDay,
        0,
        Duration::milliseconds(1),
        false,
        false,
    );
    assert!(begin < non_extended_intervals.first().unwrap().0);
    assert!(end > non_extended_intervals.last().unwrap().1);
    assert_eq!(non_extended_intervals.len(), 2);

    // The first interval's beginning should be before `begin`, but `end`
    // should be after the last interval's end
    let begin_extended_intervals = get_utc_intervals_opts(
        begin,
        end,
        &Grouping::PerDay,
        0,
        Duration::milliseconds(1),
        true,
        false,
    );
    assert!(begin > begin_extended_intervals.first().unwrap().0);
    assert!(end > begin_extended_intervals.last().unwrap().1);
    assert_eq!(begin_extended_intervals.len(), 3);

    // `begin` should be before the first interval's beginning, but the last
    // interval's end should be after `end`
    let end_extended_intervals = get_utc_intervals_opts(
        begin,
        end,
        &Grouping::PerDay,
        0,
        Duration::milliseconds(1),
        false,
        true,
    );
    assert!(begin < end_extended_intervals.first().unwrap().0);
    assert!(end < end_extended_intervals.last().unwrap().1);
    assert_eq!(end_extended_intervals.len(), 3);

    // Both `begin` and `end` should be enclosed by the intervals
    let both_extended_intervals = get_utc_intervals_opts(
        begin,
        end,
        &Grouping::PerDay,
        0,
        Duration::milliseconds(1),
        true,
        true,
    );
    assert!(begin > both_extended_intervals.first().unwrap().0);
    assert!(end < both_extended_intervals.last().unwrap().1);
    assert_eq!(both_extended_intervals.len(), 4);

    Ok(())
}

#[test]
fn test_get_utc_intervals_zero_sized() -> Result<(), Error> {
    // `end` before `begin`
    let begin = DateTime::parse_from_rfc3339("2022-11-29T08:23:45.000000Z")?;
    let end = DateTime::parse_from_rfc3339("2022-10-01T08:23:45.000000Z")?;
    assert_eq!(
        get_extended_utc_intervals_with_defaults(begin, end, &Grouping::PerDay, 0),
        vec![]
    );

    // `begin` and `end` equal
    let begin_end = DateTime::parse_from_rfc3339("2022-11-29T08:23:45.000000Z")?;
    assert_eq!(
        get_extended_utc_intervals_with_defaults(begin_end, begin_end, &Grouping::PerDay, 0),
        vec![]
    );

    // PerDay without extension on sub-day range
    let begin = DateTime::parse_from_rfc3339("2022-10-29T08:23:45.000000Z")?;
    let end = DateTime::parse_from_rfc3339("2022-10-29T10:23:45.000000Z")?;
    assert_eq!(
        get_utc_intervals_opts(
            begin,
            end,
            &Grouping::PerDay,
            0,
            Duration::milliseconds(1),
            false,
            false
        ),
        vec![]
    );

    Ok(())
}

#[test]
fn test_get_extended_utc_intervals_with_defaults() -> Result<(), Error> {
    // Regular case
    let begin = DateTime::parse_from_rfc3339("2022-10-29T08:23:45.000000Z")?;
    let end = DateTime::parse_from_rfc3339("2022-11-01T08:23:45.000000Z")?;

    let intervals =
        get_extended_utc_intervals_with_defaults(begin, end, &Grouping::PerMonth, -7200);
    dbg!(intervals);

    Ok(())
}
