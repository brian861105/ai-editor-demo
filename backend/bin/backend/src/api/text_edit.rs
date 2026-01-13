use crate::api::{errors::Error, state::AppState};
use crate::model::{Agent, PulseRequest, PulseResponse};
use crate::model::{RefineRequest, RefineResponse};
use axum::{
    Router,
    extract::{Json, State},
    routing::{get, post},
};
use backend_core::refiner::processor::{
    call_fix_api, call_improve_api, call_longer_api, call_shorter_api,
};
use backend_core::refiner::types::{RefineInput, RefineOutput};
use std::pin::Pin;
use tracing::instrument;

type RefineFuture<'a> =
    Pin<Box<dyn std::future::Future<Output = anyhow::Result<RefineOutput>> + Send + 'a>>;

pub fn routes() -> Router<AppState> {
    Router::new()
        // refine API
        .route("/improve", post(improve_text_handler))
        .route("/fix", post(fix_text_handler))
        .route("/longer", post(longer_text_handler))
        .route("/shorter", post(shorter_text_handler))
        // agent API
        .route("/agent/pulse", post(agent_pulse_handler))
        .route("/agent/list", get(list_agents_handler))
}

// refine by single task
async fn handle_refine_request<F>(
    state: &AppState,
    req: RefineRequest,
    refine_fn: F,
) -> Result<Json<RefineResponse>, Error>
where
    F: for<'a> FnOnce(RefineInput, &'a str) -> RefineFuture<'a>,
{
    let input = RefineInput { content: req.text };
    refine_fn(input, &state.api_key)
        .await
        .map(|result| {
            Json(RefineResponse {
                text: result.content,
            })
        })
        .map_err(|e| {
            tracing::error!("Refine failed: {:?}", e);
            Error::InvalidInput(e.to_string())
        })
}

/// Improve text quality and clarity.
#[instrument(skip(state, req))]
pub async fn improve_text_handler(
    State(state): State<AppState>,
    Json(req): Json<RefineRequest>,
) -> Result<Json<RefineResponse>, Error> {
    handle_refine_request(&state, req, |input, key| {
        Box::pin(call_improve_api(input, key))
    })
    .await
}

/// Fix grammar and spelling errors in text.
#[instrument(skip(state, req))]
pub async fn fix_text_handler(
    State(state): State<AppState>,
    Json(req): Json<RefineRequest>,
) -> Result<Json<RefineResponse>, Error> {
    handle_refine_request(&state, req, |input, key| Box::pin(call_fix_api(input, key))).await
}

/// Lengthen text while maintaining meaning.
#[instrument(skip(state, req))]
pub async fn longer_text_handler(
    State(state): State<AppState>,
    Json(req): Json<RefineRequest>,
) -> Result<Json<RefineResponse>, Error> {
    handle_refine_request(&state, req, |input, key| {
        Box::pin(call_longer_api(input, key))
    })
    .await
}

/// Shorten text while maintaining meaning.
#[instrument(skip(state, req))]
pub async fn shorter_text_handler(
    State(state): State<AppState>,
    Json(req): Json<RefineRequest>,
) -> Result<Json<RefineResponse>, Error> {
    handle_refine_request(&state, req, |input, key| {
        Box::pin(call_shorter_api(input, key))
    })
    .await
}

// agent API
#[instrument(skip(state, req))]
pub async fn agent_pulse_handler(
    State(state): State<AppState>,
    Json(req): Json<PulseRequest>,
) -> Result<Json<PulseResponse>, Error> {
    use backend_core::intelligence::Brain;
    use backend_core::model::PulseInput;

    let core_req = PulseInput {
        text: req.text,
        agents: req.agents.clone(),
    };

    let core_resp = Brain::evaluate_pulse(core_req, &state.api_key).await;

    let suggestions = core_resp.suggestions;

    Ok(Json(PulseResponse { suggestions }))
}

/// List all available agents.
#[instrument]
pub async fn list_agents_handler() -> Result<Json<Vec<Agent>>, Error> {
    Ok(Json(enum_iterator::all::<Agent>().collect()))
}
