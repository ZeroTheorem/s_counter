use anyhow::{Context, bail};
use chrono::{DateTime, Datelike, Duration, MappedLocalTime, NaiveDate, TimeZone, Utc};
use chrono_tz::Tz;

pub struct DateBounds {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

pub enum Period {
    ToDay,
    CurrentWeek,
    CurrentMonth,
    CurrentYear,
    ParticularMonth { year: i32, month: u32 },
    Offset { from: String, to: String },
}

fn parse_tz(localtime: MappedLocalTime<DateTime<Tz>>) -> anyhow::Result<DateTime<Utc>> {
    match localtime {
        MappedLocalTime::Single(time) => Ok(time.with_timezone(&Utc)),
        MappedLocalTime::Ambiguous(earliest, _) => Ok(earliest.with_timezone(&Utc)),
        MappedLocalTime::None => {
            bail!("The local time does not exist because there is a _gap_ in the local time")
        }
    }
}

fn get_utc(tz: Tz, date: NaiveDate) -> anyhow::Result<DateTime<Utc>> {
    let utc = tz.with_ymd_and_hms(date.year(), date.month(), date.day(), 0, 0, 0);
    parse_tz(utc)
}

pub fn period_bounds_utc(tz_str: &str, period: Period) -> anyhow::Result<DateBounds> {
    let tz: Tz = tz_str.parse().context("Invalid time zone")?;
    let now_local = Utc::now().with_timezone(&tz);

    let now_day = now_local.day();
    let now_weekday = now_local.weekday();
    let now_month = now_local.month();
    let now_year = now_local.year();

    let (start, end) = match period {
        Period::ToDay => {
            let start_day =
                NaiveDate::from_ymd_opt(now_year, now_month, now_day).context("Invalid date")?;
            let end_day = start_day + Duration::days(1);
            (start_day, end_day)
        }
        Period::CurrentWeek => {
            let days_from_monday = now_weekday.num_days_from_monday() as i64;
            let start_week = NaiveDate::from_ymd_opt(now_year, now_month, now_day)
                .context("Invalid date")?
                - Duration::days(days_from_monday);
            let end_week = start_week + Duration::days(7);
            (start_week, end_week)
        }
        Period::CurrentMonth => {
            let start_month =
                NaiveDate::from_ymd_opt(now_year, now_month, 1).context("Invalid date")?;
            let (new_year, new_month) = if now_month == 12 {
                (start_month.year() + 1, 1)
            } else {
                (start_month.year(), start_month.month() + 1)
            };
            let end_month =
                NaiveDate::from_ymd_opt(new_year, new_month, 1).context("Invalid date")?;
            (start_month, end_month)
        }
        Period::CurrentYear => {
            let start_year = NaiveDate::from_ymd_opt(now_year, 1, 1).context("Invalid date")?;
            let end_year = NaiveDate::from_ymd_opt(now_year + 1, 1, 1).context("Invalid date")?;
            (start_year, end_year)
        }
        Period::ParticularMonth { year, month } => {
            let start_month = NaiveDate::from_ymd_opt(year, month, 1).context("Invalid date")?;
            let (new_year, new_month) = if now_month == 12 {
                (start_month.year() + 1, 1)
            } else {
                (start_month.year(), start_month.month() + 1)
            };
            let end_month =
                NaiveDate::from_ymd_opt(new_year, new_month, 1).context("Invalid date")?;
            (start_month, end_month)
        }
        Period::Offset { from, to } => {
            let start = NaiveDate::parse_from_str(&from, "%Y-%m-%d")?;
            let end = NaiveDate::parse_from_str(&to, "%Y-%m-%d")?;
            (start, end)
        }
    };

    Ok(DateBounds {
        start: get_utc(tz, start)?,
        end: get_utc(tz, end)?,
    })
}
