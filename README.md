# [chrono-intervals][docsrs]: Grouped time intervals for Rust

Create chrono time intervals as "per-day", "per-week" etc.

[docsrs]: https://docs.rs/chrono-intervals

## Usage

The most convenient way to get intervals is by creating an
`IntervalGenerator`.

```rust
use chrono::{DateTime, TimeZone, Utc};
use chrono_intervals::{IntervalGenerator};

let begin = DateTime::parse_from_rfc3339("2022-06-25T08:23:45.000000Z").unwrap();
let end = DateTime::parse_from_rfc3339("2022-06-27T09:31:12.000000Z").unwrap();

let daily_intervals = IntervalGenerator::new().get_intervals(begin, end);

assert_eq!(
    daily_intervals,
    vec![
        (
            Utc.ymd(2022, 6, 25).and_hms(0, 0, 0),
            Utc.ymd(2022, 6, 25).and_hms_milli(23, 59, 59, 999),
        ),
        (
            Utc.ymd(2022, 6, 26).and_hms(0, 0, 0),
            Utc.ymd(2022, 6, 26).and_hms_milli(23, 59, 59, 999),
        ),
        (
            Utc.ymd(2022, 6, 27).and_hms(0, 0, 0),
            Utc.ymd(2022, 6, 27).and_hms_milli(23, 59, 59, 999),
        ),
    ]
);
```

The `IntervalGenerator` can be configured in many ways. Let's look at an
example of retrieving monthly intervals but in the Pacific Daylight Time
(PDT) timezone:

```rust
use chrono::{DateTime, TimeZone, Utc};
use chrono_intervals::{Grouping, IntervalGenerator};

// We want to obtain monthly intervals for month in PDT instead of in UTC.
let begin = DateTime::parse_from_rfc3339("2022-06-10T12:23:45.000000-07:00").unwrap();
let end = DateTime::parse_from_rfc3339("2022-08-26T12:23:45.000000-07:00").unwrap();

// PDT is 7h behind of UTC (towards the **west**), thus the
// `offset_west_seconds` are 7*3600
let pdt_offset_west_seconds = 7 * 3600;

let monthly_intervals = IntervalGenerator::new()
    .with_grouping(Grouping::PerMonth)
    .with_offset_west_secs(pdt_offset_west_seconds)
    .get_intervals(begin, end);

// In UTC, we expect the intervals to start 7h after the month boundary.
assert_eq!(
    monthly_intervals,
    vec![
        (
            Utc.ymd(2022, 6, 1).and_hms(7, 0, 0),
            Utc.ymd(2022, 7, 1).and_hms_milli(6, 59, 59, 999),
        ),
        (
            Utc.ymd(2022, 7, 1).and_hms(7, 0, 0),
            Utc.ymd(2022, 8, 1).and_hms_milli(6, 59, 59, 999),
        ),
        (
            Utc.ymd(2022, 8, 1).and_hms(7, 0, 0),
            Utc.ymd(2022, 9, 1).and_hms_milli(6, 59, 59, 999),
        ),
    ]
);
```

### Configuration options and defaults

Here is an overview of configurable options and their defaults:

- The interval grouping: You can choose any grouping represented in,
  `Grouping`, the default is `Grouping::PerDay`.
- The time span between the end of one interval and the beginning of the
  next (precision): This defaults to 1ms but can be overwritten by passing
  an arbitrary `chrono::Duration`. We do not check that the precision is
  reasonable. You probably want to set it to the smallest duration that you
  still consider, e.g. milliseconds or microseconds.
- The offset in seconds towards the west of your local timezone: If you want
  time intervals for e.g. Pacific Daylight Time (PDT) which is at GMT-7, you
  have to pass 7\*3600, so the time difference in seconds with a shift
  towards the west as _positive_ values. Central European Time (CET) at
  GMT+1 for example would need -3600 offset seconds towards the west.
- Whether the first interval extends to before `begin` or not: By default,
  the first interval will start on the boundary _before_ `begin`. You can
  switch this off if you want only full intervals that are strickly _after_
  `begin`.
- Whether the last interval extends to _after_ `end` or not: By default, the
  last interval will end at the boundary _after_ `end`. You can switch this
  off if you want only full intervals that are strickly _before_ `end`.

Let's look at an example with all configuration options used:

```rust
use chrono::{DateTime, Duration, TimeZone, Utc};
use chrono_intervals::{Grouping, IntervalGenerator};

let begin = DateTime::parse_from_rfc3339("2022-10-02T08:23:45.000000Z").unwrap();
let end = DateTime::parse_from_rfc3339("2022-10-18T08:23:45.000000Z").unwrap();

let inter_gen = IntervalGenerator::new()
    .with_grouping(Grouping::PerWeek)
    .with_precision(Duration::microseconds(1))
    .with_offset_west_secs(-3600)
    .without_extended_begin()
    .without_extended_end();

let weekly_intervals = inter_gen.get_intervals(begin, end);

assert_eq!(
    weekly_intervals,
    vec![
        (
            Utc.ymd(2022, 10, 2).and_hms(23, 0, 0),
            Utc.ymd(2022, 10, 9).and_hms_micro(22, 59, 59, 999999),
        ),
        (
            Utc.ymd(2022, 10, 9).and_hms(23, 0, 0),
            Utc.ymd(2022, 10, 16).and_hms_micro(22, 59, 59, 999999),
        ),
    ]
);
```

## Using functions instead of the generator

The generator is the most convenient way. However you can also use two
different functions to obtain intervals:

- `get_extended_utc_intervals` returns grouped intervals which enclose the
  `begin` and `end` and have a precision of 1ms. This is pretty close to
  the default `IntervalGenerator` behavior, just that you have to
  specify a `Grouping`.
- `get_utc_intervals_opts` returns grouped intervals and allows to specify
  all options that the generator also accepts.

### Examples

Get daily intervals between two times with default options:

```rust
use chrono::{DateTime, TimeZone, Utc};
use chrono_intervals::{Grouping, get_extended_utc_intervals};

let begin = DateTime::parse_from_rfc3339("2022-06-25T08:23:45.000000Z").unwrap();
let end = DateTime::parse_from_rfc3339("2022-06-27T09:31:12.000000Z").unwrap();

let daily_intervals =
    get_extended_utc_intervals(begin, end, &Grouping::PerDay, 0);

assert_eq!(
    daily_intervals,
    vec![
        (
            Utc.ymd(2022, 6, 25).and_hms(0, 0, 0),
            Utc.ymd(2022, 6, 25).and_hms_milli(23, 59, 59, 999),
        ),
        (
            Utc.ymd(2022, 6, 26).and_hms(0, 0, 0),
            Utc.ymd(2022, 6, 26).and_hms_milli(23, 59, 59, 999),
        ),
        (
            Utc.ymd(2022, 6, 27).and_hms(0, 0, 0),
            Utc.ymd(2022, 6, 27).and_hms_milli(23, 59, 59, 999),
        ),
    ]
);
```

Get monthly intervals with default options in the Pacific Daylight Time
(PDT) timezone:

```rust
use chrono::{DateTime, TimeZone, Utc};
use chrono_intervals::{Grouping, get_extended_utc_intervals};

// We want to obtain monthly intervals for months in PDT instead of in UTC.
let begin = DateTime::parse_from_rfc3339("2022-06-10T12:23:45.000000-07:00").unwrap();
let end = DateTime::parse_from_rfc3339("2022-08-26T12:23:45.000000-07:00").unwrap();

// PDT is 7h behind of UTC (towards the **west**), thus the
// `offset_west_seconds` are 7*3600
let pdt_offset_west_seconds = 7 * 3600;

let monthly_intervals =
    get_extended_utc_intervals(begin, end, &Grouping::PerMonth, pdt_offset_west_seconds);

// In UTC, we expect the intervals to start 7h after the day boundary.
assert_eq!(
    monthly_intervals,
    vec![
        (
            Utc.ymd(2022, 6, 1).and_hms(7, 0, 0),
            Utc.ymd(2022, 7, 1).and_hms_milli(6, 59, 59, 999),
        ),
        (
            Utc.ymd(2022, 7, 1).and_hms(7, 0, 0),
            Utc.ymd(2022, 8, 1).and_hms_milli(6, 59, 59, 999),
        ),
        (
            Utc.ymd(2022, 8, 1).and_hms(7, 0, 0),
            Utc.ymd(2022, 9, 1).and_hms_milli(6, 59, 59, 999),
        ),
    ]
);
```

Specify options for `get_utc_intervals_opts`:

```rust
use chrono::{DateTime, Duration, TimeZone, Utc};
use chrono_intervals::{Grouping, get_utc_intervals_opts};

let begin = DateTime::parse_from_rfc3339("2022-06-15T08:23:45.000000Z").unwrap();
let end = DateTime::parse_from_rfc3339("2022-06-30T09:31:12.000000Z").unwrap();

let weekly_intervals =
    get_utc_intervals_opts(
        begin,
        end,
        &Grouping::PerWeek,
        0,
        Duration::microseconds(1),  // interval end is 1µs before the next
        false,                      // start on the boundary after `start`
        true,                       // end at the boundary after `end`
    );

assert_eq!(
    weekly_intervals,
    vec![
        (
            // First interval begins **after** `begin`
            Utc.ymd(2022, 6, 20).and_hms(0, 0, 0),
            Utc.ymd(2022, 6, 26).and_hms_micro(23, 59, 59, 999999),
        ),
        (
            Utc.ymd(2022, 6, 27).and_hms(0, 0, 0),
            // Last interval ends **after** `end`
            Utc.ymd(2022, 7, 3).and_hms_micro(23, 59, 59, 999999),
        ),
    ]
);
```
