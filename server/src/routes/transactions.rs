use alloy::primitives::Address;
use anyhow::Result;
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use foundry_block_explorers::{
    Client as EtherscanClient,
    account::{NormalTransaction, Sort, TxListParams},
};
use serde::Deserialize;
use serde_json::json;
use tracing::{error, info};

use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct TxQuery {
    page: Option<u64>,
    offset: Option<u64>,
}

/// Handler for GET /wallet/:address/transactions
pub async fn get_transactions(
    Path(address): Path<Address>,
    Query(params): Query<TxQuery>,
    State(state): State<AppState>,
) -> Response {
    info!("Fetching transactions for {address}");
    let page = params.page.unwrap_or(1);
    let offset = params.offset.unwrap_or(10);

    match fetch_token_transfers(address, &state.etherscan, page, offset).await {
        Ok((transfers, has_more)) => {
            let result = json!({
                "address": format!("{address:#x}"),
                "transactions": transfers,
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
    }
}

pub async fn fetch_token_transfers(
    address: Address,
    etherscan: &EtherscanClient,
    page: u64,
    offset: u64,
) -> Result<(Vec<NormalTransaction>, bool)> {
    // Over-fetch by 1 to determine if more pages exist
    let overfetch_offset = offset + 1;

    let params = TxListParams {
        start_block: 0,
        end_block: u64::MAX,
        page,
        offset: overfetch_offset,
        sort: Sort::Desc,
    };

    let fetched = etherscan.get_transactions(&address, Some(params)).await?;
    let has_more = fetched.len() as u64 > offset;

    // Only return up to `offset` transactions
    let trimmed = if has_more {
        fetched.into_iter().take(offset as usize).collect()
    } else {
        fetched
    };

    Ok((trimmed, has_more))
}
