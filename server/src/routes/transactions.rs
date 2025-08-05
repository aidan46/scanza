use alloy::primitives::Address;
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use serde_json::json;
use tracing::{error, info, warn};

use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct TxQuery {
    page: Option<u64>,
    offset: Option<u64>,
}

pub async fn get_transactions(
    Path((chain, address)): Path<(String, Address)>,
    Query(params): Query<TxQuery>,
    State(state): State<AppState>,
) -> Response {
    info!("Getting transactions for {address} on {chain}");
    let page = params.page.unwrap_or(1);
    let offset = params.offset.unwrap_or(10);

    match state.registry.get(&chain) {
        Some(client) => match client.get_transactions(address, page, offset).await {
            Ok((transactions, has_more)) => {
                let result = json!({
                    "address": format!("{address:#x}"),
                    "transactions": transactions,
                    "pagination": {
                        "page": page,
                        "offset": offset,
                        "has_more": has_more,
                        "next_page": if has_more { Some(page + 1) } else { None }
                    }
                });
                (StatusCode::OK, Json(result)).into_response()
            }
            Err(err) => {
                error!("Failed to fetch token transfers: {err}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "error": "Failed to fetch token transfers",
                        "details": err.to_string()
                    })),
                )
                    .into_response()
            }
        },
        None => {
            warn!("Chain not found: {chain}");
            (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Chain not found"})),
            )
                .into_response()
        }
    }
}
