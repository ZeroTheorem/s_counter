use serde::Serialize;

use crate::database::Record;

#[derive(Serialize)]
#[serde(untagged)]
pub enum ApiResponse {
    RecordCounts { count: i64, period: Option<String> },
    Record(Record),
    Records(Vec<Record>),
    Success { status: String },
    Error { message: String },
}
