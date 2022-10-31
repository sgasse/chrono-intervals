use chrono::{DateTime, Datelike, Duration, TimeZone, Utc};

pub type TimeIntervalTuple = (DateTime<Utc>, DateTime<Utc>);

fn get_intervals_per_day(
    begin: DateTime<Utc>,
    end: DateTime<Utc>,
    precision: bool,
    extend_begin: bool,
    extend_end: bool,
) -> Vec<TimeIntervalTuple> {
    let mut intervals = Vec::new();

    let (mut cur_begin, mut cur_end) = match extend_begin {
        true => {
            let cur_begin = begin.date().and_hms(0, 0, 0);
            let cur_end = cur_begin + Duration::hours(24) - Duration::microseconds(1);
            (cur_begin, cur_end)
        }
        false => {
            let begin_date = begin.date();
            let cur_begin = Utc
                .ymd(begin_date.year(), begin_date.month(), begin_date.day() + 1)
                .and_hms(0, 0, 0);
            let cur_end = cur_begin + Duration::hours(24) - Duration::microseconds(1);
            (cur_begin, cur_end)
        }
    };

    while cur_end < end {
        intervals.push((cur_begin, cur_end));

        cur_begin = cur_begin + Duration::hours(24);
        cur_end = cur_begin + Duration::hours(24) - Duration::microseconds(1);
    }

    if extend_end {
        intervals.push((cur_begin, cur_end));
    }

    intervals
}

#[cfg(test)]
mod test {
    use crate::Error;
    use chrono::{DateTime, Utc};

    use super::get_intervals_per_day;

    #[test]
    fn test_get_intervals_per_day() -> Result<(), Error> {
        // Regular case
        let begin =
            DateTime::parse_from_rfc3339("2020-02-03T08:23:45.000000Z")?.with_timezone(&Utc);
        let end = DateTime::parse_from_rfc3339("2020-02-05T08:23:45.000000Z")?.with_timezone(&Utc);

        let intervals = get_intervals_per_day(begin, end, false, true, true);
        dbg!(intervals);

        Ok(())
    }
}
