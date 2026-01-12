use crate::api::{errors::Error, state::AppState};
use crate::model::{BasicRequest, BasicResponse};
use axum::{
    Router,
    extract::{Json, State},
    routing::post,
};

use backend_core::editor::processor::{
    call_fix_api, call_improve_api, call_longer_api, call_shorter_api,
};
use backend_core::editor::types::TextInput;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/improve", post(improve_text_handler))
        .route("/fix", post(improve_fix_handler))
        .route("/longer", post(longer_text_handler))
        .route("/shorter", post(shorter_text_handler))
}

pub async fn improve_text_handler(
    State(state): State<AppState>,
    Json(req): Json<BasicRequest>,
) -> Result<Json<BasicResponse>, Error> {
    let api_key = state.api_key.clone();
    let result = call_improve_api(TextInput { content: req.text }, &api_key).await;
    match result {
        Ok(result) => Ok(Json(BasicResponse {
            text: result.content,
        })),
        Err(e) => Err(Error::Custom(e.to_string())),
    }
}

pub async fn improve_fix_handler(
    State(state): State<AppState>,
    Json(req): Json<BasicRequest>,
) -> Result<Json<BasicResponse>, Error> {
    let api_key = state.api_key.clone();
    let result = call_fix_api(TextInput { content: req.text }, &api_key).await;

    match result {
        Ok(result) => Ok(Json(BasicResponse {
            text: result.content,
        })),
        Err(e) => Err(Error::Custom(e.to_string())),
    }
}

pub async fn longer_text_handler(
    State(state): State<AppState>,
    Json(req): Json<BasicRequest>,
) -> Result<Json<BasicResponse>, Error> {
    let api_key = state.api_key.clone();
    let result = call_longer_api(TextInput { content: req.text }, &api_key).await;

    match result {
        Ok(result) => Ok(Json(BasicResponse {
            text: result.content,
        })),
        Err(e) => Err(Error::Custom(e.to_string())),
    }
}

pub async fn shorter_text_handler(
    State(state): State<AppState>,
    Json(req): Json<BasicRequest>,
) -> Result<Json<BasicResponse>, Error> {
    let api_key = state.api_key.clone();

    let result = call_shorter_api(TextInput { content: req.text }, &api_key).await;

    match result {
        Ok(result) => Ok(Json(BasicResponse {
            text: result.content,
        })),
        Err(e) => Err(Error::Custom(e.to_string())),
    }
}
