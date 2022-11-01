//! Time interval computations.
mod boundaries;

use boundaries::{
    get_initial_begin_end_times_day, get_initial_begin_end_times_month,
    get_initial_begin_end_times_week, get_next_begin_end_times_day, get_next_begin_end_times_month,
    get_next_begin_end_times_week,
};
use chrono::{DateTime, Duration, FixedOffset, TimeZone};

use crate::{grouping::Grouping, TimeIntervalTuple};

pub fn get_intervals_impl<T, U>(
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
    if begin >= end {
        return Vec::with_capacity(0);
    }

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
