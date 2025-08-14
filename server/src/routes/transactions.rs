use alloy::primitives::Address;
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use foundry_block_explorers::account::NormalTransaction;
use mongodb::bson::{Document, doc, to_document};
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
                let db = state.mongodb;
                let coll_name = format!("transactions_{chain}");
                let collection = db.database("scanza").collection::<Document>(&coll_name);

                if let Err(err) = ensure_unique_hash_index(&collection).await {
                    error!("Failed to ensure unique index on {}: {err}", coll_name);
                }

                if let Err(err) =
                    upsert_transactions_by_hash(&collection, transactions.clone()).await
                {
                    error!("Failed to upsert transactions into {}: {err}", coll_name);
                }

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

async fn ensure_unique_hash_index<C: Sync + Send>(
    collection: &mongodb::Collection<C>,
) -> mongodb::error::Result<()> {
    use mongodb::{IndexModel, bson::doc, options::IndexOptions};

    let mut opts = IndexOptions::default();
    opts.unique = Some(true);
    // Only include docs where `hash` exists and is a string
    opts.partial_filter_expression = Some(doc! {
        "hash": { "$exists": true, "$type": "string" }
    });

    let index = IndexModel::builder()
        .keys(doc! { "hash": 1 })
        .options(opts)
        .build();

    collection.create_index(index).await?;
    Ok(())
}

async fn upsert_transactions_by_hash(
    collection: &mongodb::Collection<Document>,
    txs: Vec<NormalTransaction>,
) -> mongodb::error::Result<()> {
    for tx in txs {
        let doc = to_document(&tx)?;

        // Get string hash, skip bad or genesis entries
        let Some(hash_bson) = doc.get("hash") else {
            continue;
        };
        let Some(hash_str) = hash_bson.as_str() else {
            continue;
        };
        if hash_str == "GENESIS" {
            continue;
        }

        let filter = doc! { "hash": hash_str };
        let update = doc! { "$setOnInsert": doc };

        // Two-arg API, then set upsert via builder
        collection.update_one(filter, update).upsert(true).await?;
    }
    Ok(())
}
