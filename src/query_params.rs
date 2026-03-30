use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetStatsParams {
    pub period: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
}

#[derive(Deserialize)]
pub struct GetEntriesParams {
    pub year: i32,
    pub month: u32,
}
