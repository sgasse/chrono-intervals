mod common;
use chrono::{DateTime, Duration};
use chrono_intervals::{get_extended_utc_intervals, get_utc_intervals_opts, Error, Grouping};

#[test]
fn test_get_utc_intervals_zero_sized() -> Result<(), Error> {
    // `end` before `begin`
    let begin = DateTime::parse_from_rfc3339("2022-11-29T08:23:45.000000Z")?;
    let end = DateTime::parse_from_rfc3339("2022-10-01T08:23:45.000000Z")?;
    assert_eq!(
        get_extended_utc_intervals(begin, end, &Grouping::PerDay, 0),
        vec![]
    );

    // `begin` and `end` equal
    let begin_end = DateTime::parse_from_rfc3339("2022-11-29T08:23:45.000000Z")?;
    assert_eq!(
        get_extended_utc_intervals(begin_end, begin_end, &Grouping::PerDay, 0),
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

    let intervals = get_extended_utc_intervals(begin, end, &Grouping::PerMonth, -7200);
    dbg!(intervals);

    Ok(())
}
