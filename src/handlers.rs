use axum::{
    Json,
    extract::{self, Path, Query, State},
    http::StatusCode,
};
use serde_json::{Value, json};

use crate::{
    database::Database,
    query_params::{GetEntriesParams, GetStatsParams},
    requests_bodies::CreateRecordBody,
    responses::ApiResponse,
    utc::{Period, period_bounds_utc},
};

pub async fn get_stats_handler(
    State(storage): State<Database>,
    Query(params): Query<GetStatsParams>,
) -> (StatusCode, Json<Value>) {
    if let (Some(from), Some(to)) = (params.from, params.to) {
        let period_bounds =
            match period_bounds_utc("Europe/Moscow", Period::Offset { from: from, to: to }) {
                Ok(period_bounds) => period_bounds,
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"message": "Internal server error"})),
                    );
                }
            };
        match storage.count_records_for_period(period_bounds).await {
            Ok(value) => {
                return (
                    StatusCode::OK,
                    Json(json!({"count": value, "period": params.period})),
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
    let period = match params.period.as_deref() {
        Some("day") => Period::ToDay,
        Some("week") => Period::CurrentWeek,
        Some("month") => Period::CurrentMonth,
        Some("year") => Period::CurrentYear,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"message": "Bad request"})),
            );
        }
    };
    let period_bounds = match period_bounds_utc("Europe/Moscow", period) {
        Ok(period_bounds) => period_bounds,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message": "Internal server error"})),
            );
        }
    };
    match storage.count_records_for_period(period_bounds).await {
        Ok(value) => (
            StatusCode::OK,
            Json(json!({"count": value, "period": params.period})),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"message": "Internal server error"})),
        ),
    }
}
pub async fn create_record_handler<'a>(
    State(storage): State<Database>,
    extract::Json(body): extract::Json<CreateRecordBody>,
) -> (StatusCode, Json<ApiResponse>) {
    match storage.add_record(body.date, body.time).await {
        Ok(record) => {
            return (StatusCode::CREATED, Json(ApiResponse::Record(record)));
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::Error {
                    message: "Internal server error".to_string(),
                }),
            );
        }
    }
}
pub async fn get_records<'a>(
    State(storage): State<Database>,
    Query(params): Query<GetEntriesParams>,
) -> (StatusCode, Json<ApiResponse>) {
    let parsed_period = match period_bounds_utc(
        "Europe/Moscow",
        Period::ParticularMonth {
            year: params.year,
            month: params.month,
        },
    ) {
        Ok(parsed_period) => parsed_period,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::Error {
                    message: "Internal server error".to_string(),
                }),
            );
        }
    };
    match storage.get_records_for_period(parsed_period).await {
        Ok(records) => {
            return (StatusCode::OK, Json(ApiResponse::Records(records)));
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::Error {
                    message: "Internal server error".to_string(),
                }),
            );
        }
    }
}
pub async fn delete_record_handler<'a>(
    State(storage): State<Database>,
    Path(record_id): Path<i64>,
) -> (StatusCode, Json<ApiResponse>) {
    match storage.delete_record(record_id).await {
        Ok(record) => match record {
            Some(_) => {
                return (
                    StatusCode::NO_CONTENT,
                    Json(ApiResponse::Success {
                        status: "success".to_string(),
                    }),
                );
            }
            None => {
                return (
                    StatusCode::NOT_FOUND,
                    Json(ApiResponse::Error {
                        message: "Entry not found".to_string(),
                    }),
                );
            }
        },
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::Error {
                    message: "Internal server error".to_string(),
                }),
            );
        }
    }
}
