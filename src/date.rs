extern crate chrono;
use chrono::*;
fn main() {
let dt = UTC.ymd(2014, 7, 8).and_hms(9, 10, 11); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, UTC.yo(2014, 189).and_hms(9, 10, 11));
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, UTC.isoywd(2014, 28, Weekday::Tue).and_hms(9, 10, 11));

let dt = UTC.ymd(2014, 7, 8).and_hms_milli(9, 10, 11, 12); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, UTC.ymd(2014, 7, 8).and_hms_micro(9, 10, 11, 12_000));
assert_eq!(dt, UTC.ymd(2014, 7, 8).and_hms_nano(9, 10, 11, 12_000_000));

// dynamic verification
assert_eq!(UTC.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
	LocalResult::Single(UTC.ymd(2014, 7, 8).and_hms(21, 15, 33)));
assert_eq!(UTC.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(UTC.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.ymd(2014, 7, 8).and_hms_milli(9, 10, 11, 12);
let fixed_dt = FixedOffset::east(9 * 3600).ymd(2014, 7, 8).and_hms_milli(18, 10, 11, 12);
assert_eq!(dt, fixed_dt);
}