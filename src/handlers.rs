use crate::{
    database::Database,
    errors::AppError,
    query_params::{GetEntriesParams, GetStatsParams},
    requests_bodies::CreateRecordBody,
    responses::ApiResponse,
    utc::{Period, period_bounds_utc},
};
use axum::{
    Json,
    extract::{self, Path, Query, State},
    http::StatusCode,
    response::Result,
};

const TIME_ZONE: &str = "Europe/Moscow";

type Response = Result<(StatusCode, Json<ApiResponse>), AppError>;

pub async fn get_stats_handler(
    State(storage): State<Database>,
    Query(params): Query<GetStatsParams>,
) -> Response {
    let period_bounds = if let (Some(from), Some(to)) = (params.from, params.to) {
        let period_bounds = period_bounds_utc(TIME_ZONE, Period::Offset { from: from, to: to })?;
        period_bounds
    } else {
        let period = match params.period.as_deref() {
            Some("day") => Period::ToDay,
            Some("week") => Period::CurrentWeek,
            Some("month") => Period::CurrentMonth,
            Some("year") => Period::CurrentYear,
            _ => return Err(AppError::BadRequest),
        };
        let period_bounds = period_bounds_utc(TIME_ZONE, period)?;
        period_bounds
    };

    let count = storage.count_records_for_period(period_bounds).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::RecordCounts {
            count: count,
            period: params.period,
        }),
    ))
}
pub async fn create_record_handler(
    State(storage): State<Database>,
    extract::Json(body): extract::Json<CreateRecordBody>,
) -> Response {
    let record = storage.add_record(body.date, body.time).await?;
    Ok((StatusCode::CREATED, Json(ApiResponse::Record(record))))
}
pub async fn get_records(
    State(storage): State<Database>,
    Query(params): Query<GetEntriesParams>,
) -> Response {
    let parsed_period = period_bounds_utc(
        TIME_ZONE,
        Period::ParticularMonth {
            year: params.year,
            month: params.month,
        },
    )?;
    let records = storage.get_records_for_period(parsed_period).await?;
    Ok((StatusCode::OK, Json(ApiResponse::Records(records))))
}
pub async fn delete_record_handler(
    State(storage): State<Database>,
    Path(record_id): Path<i64>,
) -> Response {
    let record = storage.delete_record(record_id).await?;
    match record {
        Some(_) => {
            return Ok((
                StatusCode::NO_CONTENT,
                Json(ApiResponse::Success {
                    status: "success".to_string(),
                }),
            ));
        }
        None => {
            return Err(AppError::NotFound);
        }
    };
}
