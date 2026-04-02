use anyhow::Context;
use chrono::{DateTime, Datelike, Duration, TimeZone, Utc};
use chrono_tz::Tz;

pub struct DateBounds {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

pub enum Period {
    Day,
    Week,
    Month,
    Year,
}

pub fn period_bounds_utc(tz_str: &str, period: Period) -> anyhow::Result<DateBounds> {
    let tz: Tz = tz_str.parse().context("Invalid time zone")?;
    let now_local = Utc::now().with_timezone(&tz);

    let now_day = now_local.day();
    let now_weekday = now_local.weekday();
    let now_month = now_local.month();
    let now_year = now_local.year();

    match period {
        Period::Day => {
            let start = tz
                .with_ymd_and_hms(now_year, now_month, now_day, 0, 0, 0)
                .unwrap();
            let end = start + Duration::days(1);
            Ok(DateBounds {
                start: start.with_timezone(&Utc),
                end: end.with_timezone(&Utc),
            })
        }
        Period::Week => {
            let days_to_subtract = now_weekday.num_days_from_monday() as i64;

            let today_date = now_local.date_naive();

            let start_monday_date = today_date - Duration::days(days_to_subtract);

            let next_monday_date = start_monday_date + Duration::weeks(1);

            let start = tz
                .with_ymd_and_hms(
                    start_monday_date.year(),
                    start_monday_date.month(),
                    start_monday_date.day(),
                    0,
                    0,
                    0,
                )
                .unwrap();
            let end = tz
                .with_ymd_and_hms(
                    next_monday_date.year(),
                    next_monday_date.month(),
                    next_monday_date.day(),
                    0,
                    0,
                    0,
                )
                .unwrap();

            Ok(DateBounds {
                start: start.with_timezone(&Utc),
                end: end.with_timezone(&Utc),
            })
        }
        Period::Month => {
            let start = tz
                .with_ymd_and_hms(now_year, now_month, 1, 0, 0, 0)
                .unwrap();
            let next_month = start + Duration::days(32);
            let end = tz
                .with_ymd_and_hms(next_month.year(), next_month.month(), 1, 0, 0, 0)
                .unwrap();
            Ok(DateBounds {
                start: start.with_timezone(&Utc),
                end: end.with_timezone(&Utc),
            })
        }
        Period::Year => {
            let start = tz.with_ymd_and_hms(now_year, 1, 1, 0, 0, 0).unwrap();
            let end = tz.with_ymd_and_hms(now_year + 1, 1, 1, 0, 0, 0).unwrap();

            Ok(DateBounds {
                start: start.with_timezone(&Utc),
                end: end.with_timezone(&Utc),
            })
        }
    }
}
