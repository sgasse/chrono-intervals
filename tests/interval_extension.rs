use chrono::{DateTime, Duration};
use chrono_intervals::{get_utc_intervals_opts, grouping::Grouping, Error};

#[test]
fn test_get_utc_intervals_non_extended() -> Result<(), Error> {
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

    Ok(())
}

#[test]
fn test_get_utc_intervals_begin_extended() -> Result<(), Error> {
    let begin = DateTime::parse_from_rfc3339("2022-10-29T08:23:45.000000Z")?;
    let end = DateTime::parse_from_rfc3339("2022-11-01T08:23:45.000000Z")?;

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

    Ok(())
}

#[test]
fn test_get_utc_intervals_end_extended() -> Result<(), Error> {
    let begin = DateTime::parse_from_rfc3339("2022-10-29T08:23:45.000000Z")?;
    let end = DateTime::parse_from_rfc3339("2022-11-01T08:23:45.000000Z")?;

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

    Ok(())
}

#[test]
fn test_get_utc_intervals_both_extended() -> Result<(), Error> {
    let begin = DateTime::parse_from_rfc3339("2022-10-29T08:23:45.000000Z")?;
    let end = DateTime::parse_from_rfc3339("2022-11-01T08:23:45.000000Z")?;

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
