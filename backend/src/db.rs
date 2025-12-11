use chrono::{
    DateTime,
    TimeZone,
};
use libsql::Value;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct DateTimeValue<'a, TZ>(&'a DateTime<TZ>)
    where TZ: TimeZone;

impl<'a, TZ> From<&'a DateTime<TZ>> for DateTimeValue<'a, TZ>
where TZ: TimeZone {
    fn from(date_time: &'a DateTime<TZ>) -> Self {
        Self(date_time)
    }
}

impl<'a, TZ> From<DateTimeValue<'a, TZ>> for Value
where TZ: TimeZone {
    fn from(date_time: DateTimeValue<TZ>) -> Self {
        Self::Text(date_time.0.to_rfc3339())
    }
}

