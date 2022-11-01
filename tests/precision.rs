use chrono::{DateTime, Duration, NaiveTime};
use chrono_intervals::{get_utc_intervals_opts, grouping::Grouping, Error};

#[test]
fn test_get_utc_intervals_precision_millis() -> Result<(), Error> {
    let begin = DateTime::parse_from_rfc3339("2022-10-29T08:23:45.000000Z")?;
    let end = DateTime::parse_from_rfc3339("2022-11-01T08:23:45.000000Z")?;

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

    Ok(())
}

#[test]
fn test_get_utc_intervals_precision_micros() -> Result<(), Error> {
    let begin = DateTime::parse_from_rfc3339("2022-10-29T08:23:45.000000Z")?;
    let end = DateTime::parse_from_rfc3339("2022-11-01T08:23:45.000000Z")?;

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

    Ok(())
}

#[test]
fn test_get_utc_intervals_precision_nanos() -> Result<(), Error> {
    let begin = DateTime::parse_from_rfc3339("2022-10-29T08:23:45.000000Z")?;
    let end = DateTime::parse_from_rfc3339("2022-11-01T08:23:45.000000Z")?;

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
