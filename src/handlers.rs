use axum::{
    Json,
    extract::{self, Path, Query, State},
    http::StatusCode,
};
use serde_json::{Value, json};

use crate::{
    database::Database,
    parser::{parse_date_period, parse_period_from_ym},
    query_params::{GetEntriesParams, GetStatsParams},
    requests_bodies::CreateRecordBody,
};

pub async fn get_stats_handler(
    State(storage): State<Database>,
    Query(params): Query<GetStatsParams>,
) -> (StatusCode, Json<Value>) {
    if let (Some(from), Some(to)) = (params.from, params.to) {
        let parsed_period = match parse_date_period(&from, &to) {
            Ok(parsed_period) => parsed_period,
            Err(_) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"message": "Invalid date"})),
                );
            }
        };
        match storage
            .get_records_from_to(parsed_period.from, parsed_period.to)
            .await
        {
            Ok(value) => {
                return (
                    StatusCode::OK,
                    Json(json!({"count": value, "period": params.period, "from": from, "to": to})),
                );
            }
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"message": "Internal server error"})),
                );
            }
        }
    }
    match params.period.as_deref() {
        Some("day") => match storage.get_current_day_records().await {
            Ok(value) => (
                StatusCode::OK,
                Json(json!({"count": value, "period": params.period})),
            ),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal server error"})),
            ),
        },

        Some("week") => match storage.get_current_week_records().await {
            Ok(value) => (
                StatusCode::OK,
                Json(json!({"count": value, "period": params.period})),
            ),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal server error"})),
            ),
        },

        Some("month") => match storage.get_current_month_records().await {
            Ok(value) => (
                StatusCode::OK,
                Json(json!({"count": value, "period": params.period})),
            ),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal server error"})),
            ),
        },
        Some("year") => match storage.get_current_year_records().await {
            Ok(value) => (
                StatusCode::OK,
                Json(json!({"count": value, "period": params.period})),
            ),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal server error"})),
            ),
        },
        _ => (
            StatusCode::BAD_REQUEST,
            Json(json!({"message": "Bad request"})),
        ),
    }
}

pub async fn create_record_handler(
    State(storage): State<Database>,
    extract::Json(body): extract::Json<CreateRecordBody>,
) -> (StatusCode, Json<Value>) {
    match storage.add_record(body.date, body.time).await {
        Ok(record) => {
            return (StatusCode::CREATED, Json(json!(record)));
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal server error"})),
            );
        }
    }
}
pub async fn get_entries(
    State(storage): State<Database>,
    Query(params): Query<GetEntriesParams>,
) -> (StatusCode, Json<Value>) {
    let parsed_period = match parse_period_from_ym(params.year, params.month) {
        Ok(parsed_period) => parsed_period,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"message": "Invalid date"})),
            );
        }
    };
    match storage
        .get_entries_from_to(parsed_period.from, parsed_period.to)
        .await
    {
        Ok(records) => {
            return (StatusCode::OK, Json(json!(records)));
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal server error"})),
            );
        }
    }
}
pub async fn delete_record_handler(
    State(storage): State<Database>,
    Path(record_id): Path<i64>,
) -> (StatusCode, Json<Value>) {
    match storage.delete_record(record_id).await {
        Ok(record) => match record {
            Some(_) => {
                return (StatusCode::NO_CONTENT, Json(json!({})));
            }
            None => {
                return (
                    StatusCode::NOT_FOUND,
                    Json(json!({"message": "Entry not found"})),
                );
            }
        },
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal server error"})),
            );
        }
    }
}
