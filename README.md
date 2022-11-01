# [chrono-intervals][docsrs]: Grouped time intervals for Rust

Create chrono time intervals as "per-day", "per-week" etc.

[docsrs]: https://docs.rs/chrono-intervals

## Defaults and options

- Only timezone-aware `Datetime` objects can be passed in and retrieved.
- All retrieved intervals are in the UTC timezone.
- By default, the first and last interval are extended to _enclose_ the
  specified beginning and ending. Both extensions can be switched off separately
  with options.
- By default, the end of an interval is 1ms before the start of the next
  interval. The precision of this can be specified with options.
