use chrono::{DateTime, Duration, FixedOffset, TimeZone, Utc};

use crate::{grouping::Grouping, intervals_impl::get_intervals_impl, TimeIntervalTuple};

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
