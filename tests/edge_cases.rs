mod common;
use chrono::{DateTime, Duration};
use chrono_intervals::{
    get_extended_utc_intervals, get_utc_intervals_opts, Error, Grouping, IntervalGenerator,
};

#[test]
fn test_get_utc_intervals_zero_sized() -> Result<(), Error> {
    // `end` before `begin`
    let begin = DateTime::parse_from_rfc3339("2022-11-29T08:23:45.000000Z")?;
    let end = DateTime::parse_from_rfc3339("2022-10-01T08:23:45.000000Z")?;
    assert_eq!(
        IntervalGenerator::new().get_intervals(begin, end),
        Vec::with_capacity(0)
    );
    assert_eq!(
        get_extended_utc_intervals(begin, end, &Grouping::PerDay, 0),
        Vec::with_capacity(0)
    );

    // `begin` and `end` equal
    let begin_end = DateTime::parse_from_rfc3339("2022-11-29T08:23:45.000000Z")?;
    assert_eq!(
        IntervalGenerator::new().get_intervals(begin_end, begin_end),
        Vec::with_capacity(0)
    );
    assert_eq!(
        get_extended_utc_intervals(begin_end, begin_end, &Grouping::PerDay, 0),
        Vec::with_capacity(0)
    );

    // PerDay without extension on sub-day range
    let begin = DateTime::parse_from_rfc3339("2022-10-29T08:23:45.000000Z")?;
    let end = DateTime::parse_from_rfc3339("2022-10-29T10:23:45.000000Z")?;
    assert_eq!(
        IntervalGenerator::new()
            .without_extension()
            .get_intervals(begin, end),
        Vec::with_capacity(0)
    );
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
        Vec::with_capacity(0)
    );

    Ok(())
}
