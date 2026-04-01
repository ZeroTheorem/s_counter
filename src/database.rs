use anyhow::Context;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Serialize;
use sqlx::{PgPool, postgres::PgPoolOptions};

#[derive(Clone, Debug)]
pub struct Database {
    pool: PgPool,
}

#[derive(Serialize)]
pub struct Record {
    id: i64,
    date: String,
    time: String,
    duration: Option<i32>,
    notes: Option<String>,
    created_at: DateTime<Utc>,
}
impl Database {
    pub async fn builder() -> anyhow::Result<Self> {
        let database_url = &std::env::var("DATABASE_URL").context("DATABASE_URL not found")?;
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .context("error while migrate")?;
        Ok(Database { pool })
    }
    pub async fn get_current_day_records(&self) -> anyhow::Result<i64> {
        let result = sqlx::query!(
            "SELECT COUNT(*)
             FROM sex
             WHERE created_at >= date_trunc('day', now() AT TIME ZONE 'Europe/Moscow')
             AND created_at < date_trunc('day', now() AT TIME ZONE 'Europe/Moscow') + interval '1 day';"
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(result.count.unwrap_or(0))
    }
    pub async fn get_current_week_records(&self) -> anyhow::Result<i64> {
        let result = sqlx::query!(
            "SELECT COUNT(*)
             FROM sex
             WHERE created_at >= date_trunc('week', now() AT TIME ZONE 'Europe/Moscow')
             AND created_at < date_trunc('week', now() AT TIME ZONE 'Europe/Moscow') + interval '1 week';"
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(result.count.unwrap_or(0))
    }
    pub async fn get_current_month_records(&self) -> anyhow::Result<i64> {
        let result = sqlx::query!(
            "SELECT COUNT(*)
             FROM sex
             WHERE created_at >= date_trunc('month', now() AT TIME ZONE 'Europe/Moscow')
             AND created_at < date_trunc('month', now() AT TIME ZONE 'Europe/Moscow') + interval '1 month';"
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(result.count.unwrap_or(0))
    }
    pub async fn get_current_year_records(&self) -> anyhow::Result<i64> {
        let result = sqlx::query!(
            "SELECT COUNT(*)
             FROM sex
             WHERE created_at >= date_trunc('year', now() AT TIME ZONE 'Europe/Moscow')
             AND created_at < date_trunc('year', now() AT TIME ZONE 'Europe/Moscow') + interval '1 year';"
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(result.count.unwrap_or(0))
    }
    pub async fn get_records_from_to(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> anyhow::Result<i64> {
        let result = sqlx::query!(
            "SELECT COUNT(*)
             FROM sex
             WHERE created_at >= $1
             AND created_at < $2",
            from,
            to
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(result.count.unwrap_or(0))
    }
    pub async fn add_record(&self, date: String, time: String) -> anyhow::Result<Record> {
        let record = sqlx::query_as!(
            Record,
            "INSERT INTO sex (date, time) VALUES ($1, $2) RETURNING *",
            date,
            time
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(record)
    }
    pub async fn get_entries_from_to(
        &self,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> anyhow::Result<Vec<Record>> {
        let result = sqlx::query_as!(
            Record,
            "SELECT *
             FROM sex
             WHERE created_at >= $1
             AND created_at < $2
             ORDER BY created_at DESC",
            from,
            to
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(result)
    }
    pub async fn delete_record(&self, record_id: i64) -> anyhow::Result<Option<i64>> {
        let result = sqlx::query!("DELETE FROM sex WHERE id = $1 RETURNING id", record_id)
            .fetch_optional(&self.pool)
            .await?;
        match result {
            Some(result) => Ok(Some(result.id)),
            None => Ok(None),
        }
    }
}
