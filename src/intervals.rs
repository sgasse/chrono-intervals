use chrono::{DateTime, Datelike, Duration, FixedOffset, TimeZone, Utc};

pub type TimeIntervalTuple<T> = (DateTime<T>, DateTime<T>);

fn get_intervals_per_day<T, U>(
    begin: DateTime<T>,
    end: DateTime<T>,
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
    let tz = local_timezone;

    let mut intervals = Vec::new();
    let (mut cur_begin, mut cur_end) = match extend_begin {
        true => {
            let cur_begin = begin.with_timezone(tz).date().and_hms(0, 0, 0);
            let cur_end = cur_begin + Duration::hours(24) - end_precision;
            (cur_begin, cur_end)
        }
        false => {
            let begin_date = begin.date();
            let cur_begin = tz
                .ymd(begin_date.year(), begin_date.month(), begin_date.day() + 1)
                .and_hms(0, 0, 0);
            let cur_end = cur_begin + Duration::hours(24) - end_precision;
            (cur_begin, cur_end)
        }
    };

    while cur_end < end {
        intervals.push((cur_begin, cur_end));

        cur_begin = cur_begin + Duration::hours(24);
        cur_end = cur_begin + Duration::hours(24) - end_precision;
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

#[cfg(test)]
mod test {
    use crate::Error;
    use chrono::{DateTime, Duration, FixedOffset, Utc};

    use super::get_intervals_per_day;

    #[test]
    fn test_get_intervals_per_day() -> Result<(), Error> {
        // Regular case
        let begin = DateTime::parse_from_rfc3339("2020-02-03T08:23:45.000000Z")?;
        let end = DateTime::parse_from_rfc3339("2020-02-05T08:23:45.000000Z")?;

        let local_timezone = FixedOffset::west(-7200);
        let intervals = get_intervals_per_day(
            begin,
            end,
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
