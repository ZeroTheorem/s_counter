use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateRecordBody {
    pub date: String,
    pub time: String,
    #[allow(dead_code)]
    duration: Option<i64>,
    #[allow(dead_code)]
    notes: Option<String>,
}
