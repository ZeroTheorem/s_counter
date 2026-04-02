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
    responses::ApiResponse,
    utc::{Period, period_bounds_utc},
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
        Some("day") => {
            let period_bounds = match period_bounds_utc("Europe/Moscow", Period::Day) {
                Ok(period_bounds) => period_bounds,
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"message": "Internal server error"})),
                    );
                }
            };
            match storage.get_records_from_period(period_bounds).await {
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

        Some("week") => {
            let period_bounds = match period_bounds_utc("Europe/Moscow", Period::Week) {
                Ok(period_bounds) => period_bounds,
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"message": "Internal server error"})),
                    );
                }
            };
            match storage.get_records_from_period(period_bounds).await {
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

        Some("month") => {
            let period_bounds = match period_bounds_utc("Europe/Moscow", Period::Month) {
                Ok(period_bounds) => period_bounds,
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"message": "Internal server error"})),
                    );
                }
            };
            match storage.get_records_from_period(period_bounds).await {
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
        Some("year") => {
            let period_bounds = match period_bounds_utc("Europe/Moscow", Period::Year) {
                Ok(period_bounds) => period_bounds,
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"message": "Internal server error"})),
                    );
                }
            };
            match storage.get_records_from_period(period_bounds).await {
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
        _ => (
            StatusCode::BAD_REQUEST,
            Json(json!({"message": "Bad request"})),
        ),
    }
}

pub async fn create_record_handler<'a>(
    State(storage): State<Database>,
    extract::Json(body): extract::Json<CreateRecordBody>,
) -> (StatusCode, Json<ApiResponse<'a>>) {
    match storage.add_record(body.date, body.time).await {
        Ok(record) => {
            return (StatusCode::CREATED, Json(ApiResponse::Record(record)));
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::Error {
                    message: "Internal server error",
                }),
            );
        }
    }
}
pub async fn get_entries<'a>(
    State(storage): State<Database>,
    Query(params): Query<GetEntriesParams>,
) -> (StatusCode, Json<ApiResponse<'a>>) {
    let parsed_period = match parse_period_from_ym(params.year, params.month) {
        Ok(parsed_period) => parsed_period,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::Error {
                    message: "Internal server error",
                }),
            );
        }
    };
    match storage
        .get_entries_from_to(parsed_period.from, parsed_period.to)
        .await
    {
        Ok(records) => {
            return (StatusCode::OK, Json(ApiResponse::Records(records)));
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::Error {
                    message: "Internal server error",
                }),
            );
        }
    }
}
pub async fn delete_record_handler<'a>(
    State(storage): State<Database>,
    Path(record_id): Path<i64>,
) -> (StatusCode, Json<ApiResponse<'a>>) {
    match storage.delete_record(record_id).await {
        Ok(record) => match record {
            Some(_) => {
                return (
                    StatusCode::NO_CONTENT,
                    Json(ApiResponse::Success { status: "success" }),
                );
            }
            None => {
                return (
                    StatusCode::NOT_FOUND,
                    Json(ApiResponse::Error {
                        message: "Entry not found",
                    }),
                );
            }
        },
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::Error {
                    message: "Internal server error",
                }),
            );
        }
    }
}
