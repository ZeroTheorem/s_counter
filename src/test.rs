use anyhow::{Context, bail};
use chrono::{DateTime, Datelike, Duration, LocalResult, NaiveDate, TimeZone, Utc};
use chrono_tz::Tz;

#[derive(Debug, Clone)]
pub struct DateBounds {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy)]
pub enum Period {
    Day,
    Week,
    Month,
    Year,
}

/// Безопасно преобразует LocalResult в DateTime, обрабатывая DST.
/// При неоднозначности (часы назад) берём более раннее время,
/// при несуществующем (часы вперёд) — ошибка.
fn resolve_local(result: LocalResult<DateTime<Tz>>, label: &str) -> anyhow::Result<DateTime<Utc>> {
    match result {
        LocalResult::Single(dt) => Ok(dt.with_timezone(&Utc)),
        LocalResult::Ambiguous(earliest, _latest) => Ok(earliest.with_timezone(&Utc)),
        LocalResult::None => bail!("Non-existent local time for '{}' (DST gap)", label),
    }
}

fn midnight_utc(tz: Tz, date: NaiveDate) -> anyhow::Result<DateTime<Utc>> {
    let result = tz.with_ymd_and_hms(date.year(), date.month(), date.day(), 0, 0, 0);
    resolve_local(result, &date.to_string())
}

pub fn period_bounds_utc(tz_str: &str, period: Period) -> anyhow::Result<DateBounds> {
    let tz: Tz = tz_str.parse().context("Invalid time zone")?;
    let now_local = Utc::now().with_timezone(&tz);
    let today = now_local.date_naive();

    let (start_date, end_date) = match period {
        Period::Day => (today, today + Duration::days(1)),

        Period::Week => {
            let days_back = now_local.weekday().num_days_from_monday() as i64;
            let monday = today - Duration::days(days_back);
            (monday, monday + Duration::weeks(1))
        }

        Period::Month => {
            let first = NaiveDate::from_ymd_opt(today.year(), today.month(), 1)
                .context("Invalid month start")?;
            let (next_year, next_month) = if today.month() == 12 {
                (today.year() + 1, 1)
            } else {
                (today.year(), today.month() + 1)
            };
            let first_next = NaiveDate::from_ymd_opt(next_year, next_month, 1)
                .context("Invalid next month start")?;
            (first, first_next)
        }

        Period::Year => {
            let first =
                NaiveDate::from_ymd_opt(today.year(), 1, 1).context("Invalid year start")?;
            let first_next = NaiveDate::from_ymd_opt(today.year() + 1, 1, 1)
                .context("Invalid next year start")?;
            (first, first_next)
        }
    };

    Ok(DateBounds {
        start: midnight_utc(tz, start_date)?,
        end: midnight_utc(tz, end_date)?,
    })
}
