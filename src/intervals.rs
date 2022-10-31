//! Time interval computations.
use crate::{grouping::Grouping, TimeIntervalTuple};
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

fn get_intervals_impl<T, U>(
    begin: DateTime<T>,
    end: DateTime<T>,
    grouping: &Grouping,
    end_precision: Duration,
    local_timezone: &FixedOffset,
    output_timezone: &U,
    extend_begin: bool,
    extend_end: bool,
) -> Vec<TimeIntervalTuple<U>>
where
    T: TimeZone,
    U: TimeZone,
{
    let mut intervals = Vec::new();
    let (mut cur_begin, mut cur_end) = match grouping {
        Grouping::PerDay => {
            get_initial_begin_end_times_day(begin, local_timezone, end_precision, extend_begin)
        }
        Grouping::PerWeek => {
            get_initial_begin_end_times_week(begin, local_timezone, end_precision, extend_begin)
        }
        Grouping::PerMonth => {
            get_initial_begin_end_times_month(begin, local_timezone, end_precision, extend_begin)
        }
    };

    while cur_end < end {
        intervals.push((cur_begin, cur_end));

        (cur_begin, cur_end) = match grouping {
            Grouping::PerDay => get_next_begin_end_times_day(cur_begin, end_precision),
            Grouping::PerWeek => get_next_begin_end_times_week(cur_begin, end_precision),
            Grouping::PerMonth => get_next_begin_end_times_month(cur_begin, end_precision),
        }
    }

    if extend_end {
        intervals.push((cur_begin, cur_end));
    }

    intervals
        .into_iter()
        .map(|interval| {
            (
                interval.0.with_timezone(output_timezone),
                interval.1.with_timezone(output_timezone),
            )
        })
        .collect()
}

fn get_initial_begin_end_times_day<T>(
    begin: DateTime<T>,
    local_timezone: &FixedOffset,
    end_precision: Duration,
    extend_begin: bool,
) -> (DateTime<FixedOffset>, DateTime<FixedOffset>)
where
    T: TimeZone,
{
    let init_begin = match extend_begin {
        true => begin.with_timezone(local_timezone).date().and_hms(0, 0, 0),
        false => begin.with_timezone(local_timezone).date().and_hms(0, 0, 0) + Duration::hours(24),
    };
    let init_end = init_begin + Duration::hours(24) - end_precision;
    (init_begin, init_end)
}

fn get_initial_begin_end_times_week<T>(
    begin: DateTime<T>,
    local_timezone: &FixedOffset,
    end_precision: Duration,
    extend_begin: bool,
) -> (DateTime<FixedOffset>, DateTime<FixedOffset>)
where
    T: TimeZone,
{
    let localized_begin = begin.with_timezone(local_timezone);
    let num_days_since_monday = localized_begin.weekday() as i64;
    let init_begin = match extend_begin {
        true => localized_begin.date().and_hms(0, 0, 0) - Duration::days(num_days_since_monday),
        false => {
            localized_begin.date().and_hms(0, 0, 0) + Duration::days(7 - num_days_since_monday)
        }
    };
    let init_end = init_begin + Duration::days(7) - end_precision;
    (init_begin, init_end)
}

fn get_initial_begin_end_times_month<T>(
    begin: DateTime<T>,
    local_timezone: &FixedOffset,
    end_precision: Duration,
    extend_begin: bool,
) -> (DateTime<FixedOffset>, DateTime<FixedOffset>)
where
    T: TimeZone,
{
    let localized_begin = begin.with_timezone(local_timezone);
    let init_begin = match extend_begin {
        true => local_timezone
            .ymd(localized_begin.year(), localized_begin.month(), 1)
            .and_hms(0, 0, 0),
        false => next_month_start(localized_begin),
    };
    let init_end = next_month_start(init_begin) - end_precision;
    (init_begin, init_end)
}

fn get_next_begin_end_times_day(
    cur_begin: DateTime<FixedOffset>,
    end_precision: Duration,
) -> (DateTime<FixedOffset>, DateTime<FixedOffset>) {
    let cur_begin = cur_begin + Duration::hours(24);
    let cur_end = cur_begin + Duration::hours(24) - end_precision;
    (cur_begin, cur_end)
}

fn get_next_begin_end_times_week(
    cur_begin: DateTime<FixedOffset>,
    end_precision: Duration,
) -> (DateTime<FixedOffset>, DateTime<FixedOffset>) {
    let cur_begin = cur_begin + Duration::days(7);
    let cur_end = cur_begin + Duration::days(7) - end_precision;
    (cur_begin, cur_end)
}

fn get_next_begin_end_times_month(
    cur_begin: DateTime<FixedOffset>,
    end_precision: Duration,
) -> (DateTime<FixedOffset>, DateTime<FixedOffset>) {
    let cur_begin = next_month_start(cur_begin);
    let cur_end = next_month_start(cur_begin) - end_precision;
    (cur_begin, cur_end)
}

fn next_month_start<T>(datetime: DateTime<T>) -> DateTime<T>
where
    T: TimeZone,
{
    let date = datetime.date();
    datetime
        .timezone()
        .ymd(
            match date.month() {
                12 => date.year() + 1,
                _ => date.year(),
            },
            match date.month() {
                12 => 1,
                _ => date.month() + 1,
            },
            1,
        )
        .and_hms(0, 0, 0)
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
