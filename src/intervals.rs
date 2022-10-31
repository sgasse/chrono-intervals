use crate::grouping::Grouping;
use chrono::{DateTime, Datelike, Duration, FixedOffset, TimeZone};

pub type TimeIntervalTuple<T> = (DateTime<T>, DateTime<T>);

fn get_intervals<T, U>(
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
    use crate::Error;
    use chrono::{DateTime, Duration, FixedOffset, Utc};

    use super::get_intervals;

    #[test]
    fn test_get_intervals_per_day() -> Result<(), Error> {
        // Regular case
        let begin = DateTime::parse_from_rfc3339("2022-10-29T08:23:45.000000Z")?;
        let end = DateTime::parse_from_rfc3339("2022-11-01T08:23:45.000000Z")?;

        let local_timezone = FixedOffset::west(-7200);
        let intervals = get_intervals(
            begin,
            end,
            &crate::grouping::Grouping::PerMonth,
            Duration::microseconds(1),
            &local_timezone,
            &Utc,
            true,
            true,
        );
        dbg!(intervals);

        Ok(())
    }
}
