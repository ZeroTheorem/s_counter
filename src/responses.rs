use serde::Serialize;

use crate::database::Record;

#[derive(Serialize)]
#[serde(untagged)]
pub enum ApiResponse<'a> {
    Record(Record),
    Records(Vec<Record>),
    Success { status: &'a str },
    Error { message: &'a str },
}
