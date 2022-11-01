use chrono::{DateTime, Datelike, Duration, FixedOffset, TimeZone};

pub fn get_initial_begin_end_times_day<T>(
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

pub fn get_initial_begin_end_times_week<T>(
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

pub fn get_initial_begin_end_times_month<T>(
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

pub fn get_next_begin_end_times_day(
    cur_begin: DateTime<FixedOffset>,
    end_precision: Duration,
) -> (DateTime<FixedOffset>, DateTime<FixedOffset>) {
    let cur_begin = cur_begin + Duration::hours(24);
    let cur_end = cur_begin + Duration::hours(24) - end_precision;
    (cur_begin, cur_end)
}

pub fn get_next_begin_end_times_week(
    cur_begin: DateTime<FixedOffset>,
    end_precision: Duration,
) -> (DateTime<FixedOffset>, DateTime<FixedOffset>) {
    let cur_begin = cur_begin + Duration::days(7);
    let cur_end = cur_begin + Duration::days(7) - end_precision;
    (cur_begin, cur_end)
}

pub fn get_next_begin_end_times_month(
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
