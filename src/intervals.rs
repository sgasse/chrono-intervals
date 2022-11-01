use crate::{grouping::Grouping, intervals_impl::get_intervals_impl, TimeIntervalTuple};
use chrono::{DateTime, Datelike, Duration, FixedOffset, TimeZone, Utc};

/// Get time intervals with options in the UTC timezone.
///
/// - With `offset_west_seconds`, the intervals boundaries (begin of a day,
///   week, month etc.) are shifted towards the west, allowing to retrieve e.g.
///   day intervals starting on the day boundary in a local timezone.
/// - `end_precision` determines how much time before an interval boundary the
///   previous interval ends.
/// - If `extend_begin` is `true`, the first intervals starts on the interval
///   boundary **before** `begin`, otherwise on the interval boundary after it.
/// - If `extend_end` is `true`, the last intervals ends at the interval
///   boundary **after** `end`, otherwise before it.
pub fn get_utc_intervals_opts<T>(
    begin: DateTime<T>,
    end: DateTime<T>,
    grouping: &Grouping,
    offset_west_seconds: i32,
    end_precision: Duration,
    extend_begin: bool,
    extend_end: bool,
) -> Vec<TimeIntervalTuple<Utc>>
where
    T: TimeZone,
{
    let local_timezone = &FixedOffset::west(offset_west_seconds);
    get_intervals_impl(
        begin,
        end,
        grouping,
        end_precision,
        local_timezone,
        &Utc,
        extend_begin,
        extend_end,
    )
}

/// Get extended time intervals with default options in the UTC timezone.
///
/// - Extended means that the first interval starts on the interval boundary
///   **before** `begin` and the last one ends on the interval boundary
///   **after** end.
/// - The default `end_precision` is 1ms, so the interval end is always 1ms
///   before the next interval boundary.
/// - Interval boundaries are shifted by `offset_west_seconds`. This allows to
///   retrieve e.g. daily intervals starting with the days in a specific time
///   zone.
pub fn get_extended_utc_intervals_with_defaults<T>(
    begin: DateTime<T>,
    end: DateTime<T>,
    grouping: &Grouping,
    offset_west_seconds: i32,
) -> Vec<TimeIntervalTuple<Utc>>
where
    T: TimeZone,
{
    let local_timezone = &FixedOffset::west(offset_west_seconds);
    get_intervals_impl(
        begin,
        end,
        grouping,
        Duration::milliseconds(1),
        local_timezone,
        &Utc,
        true,
        true,
    )
}

#[cfg(test)]
mod test {
    use super::{get_extended_utc_intervals_with_defaults, get_utc_intervals_opts};
    use crate::{grouping::Grouping, Error};
    use chrono::{DateTime, Duration, FixedOffset, NaiveTime, TimeZone, Timelike, Utc};
    use rand::Rng;

    fn random_time(start_year: i32) -> DateTime<Utc> {
        let mut rng = rand::thread_rng();
        Utc.ymd(
            start_year + rng.gen_range(0..100),
            rng.gen_range(1..=12),
            rng.gen_range(1..=28),
        )
        .and_hms(
            rng.gen_range(0..24),
            rng.gen_range(0..60),
            rng.gen_range(0..60),
        )
    }

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
    fn test_get_extended_utc_intervals_with_defaults() -> Result<(), Error> {
        // Regular case
        let begin = DateTime::parse_from_rfc3339("2022-10-29T08:23:45.000000Z")?;
        let end = DateTime::parse_from_rfc3339("2022-11-01T08:23:45.000000Z")?;

        let intervals =
            get_extended_utc_intervals_with_defaults(begin, end, &Grouping::PerMonth, -7200);
        dbg!(intervals);

        Ok(())
    }
}
