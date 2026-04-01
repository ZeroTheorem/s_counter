use anyhow::anyhow;
use chrono::{DateTime, Months, NaiveDate, Utc};

use tracing::info;

pub struct ParsedPeriod {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
}

pub fn parse_date_period(from: &str, to: &str) -> anyhow::Result<ParsedPeriod> {
    let parsed_from = NaiveDate::parse_from_str(&from, "%Y-%m-%d")?
        .and_hms_opt(0, 0, 0)
        .ok_or_else(|| anyhow!("Invalid time"))?;
    let parsed_to = NaiveDate::parse_from_str(&to, "%Y-%m-%d")?
        .and_hms_opt(0, 0, 0)
        .ok_or_else(|| anyhow!("Invalid time"))?;
    Ok(ParsedPeriod {
        from: parsed_from.and_utc(),
        to: parsed_to.and_utc(),
    })
}

pub fn parse_period_from_ym(year: i32, month: u32) -> anyhow::Result<ParsedPeriod> {
    let parsed_from = NaiveDate::from_ymd_opt(year, month, 1)
        .ok_or_else(|| anyhow!("Invalid date"))?
        .and_hms_opt(0, 0, 0)
        .ok_or_else(|| anyhow!("Invalid time"))?;
    let parsed_to = parsed_from
        .date()
        .checked_add_months(Months::new(1))
        .ok_or_else(|| anyhow!("Failed add month to date"))?
        .and_hms_opt(0, 0, 0)
        .ok_or_else(|| anyhow!("Invalid time"))?;
    info!("{} ---- {}", parsed_from, parsed_to);
    Ok(ParsedPeriod {
        from: parsed_from.and_utc(),
        to: parsed_to.and_utc(),
    })
}
