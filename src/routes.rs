use actix_web::{
    HttpRequest, HttpResponse,
    http::header,
    web::{self, Bytes},
};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{app_state::AppState, errors::AppError, validators::ValidatorSummary};

const MAX_UPSTREAM_BODY: usize = 32 * 1024 * 1024; // 32 MiB

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/health").route(web::get().to(health_check)))
            .service(web::resource("/validators").route(web::get().to(list_validators)))
            .service(web::resource("/")
                .route(web::get().to(index_info))
                .route(web::post().to(proxy_rpc))
            );
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse { status: "ok" })
    }

async fn index_info() -> HttpResponse {
    HttpResponse::Ok().json(IndexInfo {
        name: "TheSolanaApi",
        description: "Provides a single, stable access point to a fleet of Solana validators. The API accepts standard Solana JSON-RPC requests and routes them to an available validator based on your selection criteria.",
        docs: "https://github.com/BastienGimbert/TheSolanaApi",
        usage: "POST /?server=<name>, /?location=<region>, or / for a random location with a Solana JSON-RPC body. See /validators for options.",
        health: "/health",
        validators: "/validators",
        example: "curl -X POST 'http://thesolanaapi.com/?server=frankfurt-1' -H 'Content-Type: application/json' -d '{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"getVersion\",\"params\":[]}'",
    })
}

    #[derive(serde::Serialize)]
    struct IndexInfo {
        name: &'static str,
        description: &'static str,
        docs: &'static str,
        usage: &'static str,
        health: &'static str,
        validators: &'static str,
        example: &'static str,
    }


async fn list_validators(state: web::Data<AppState>) -> HttpResponse {
    let validators = state.registry().summaries();
    HttpResponse::Ok().json(ValidatorsResponse { validators })
}

async fn proxy_rpc(
    state: web::Data<AppState>,
    req: HttpRequest,
    body: Bytes,
    query: web::Query<ProxyQuery>,
) -> Result<HttpResponse, AppError> {
    let selected = state
        .registry()
        .select(query.validator.as_deref(), query.location.as_deref())?;

    info!(
        validator = selected.name(),
        location = selected.location(),
        "forwarding json-rpc request"
    );

    let client = state.build_client();

    let mut forward_req = client.request_from(selected.rpc_url().as_str(), req.head());

    if let Some(host) = selected.host_header() {
        forward_req = forward_req.insert_header((header::HOST, host));
    }
    
    let mut upstream_resp = match forward_req.send_body(body).await {
        Ok(resp) => resp,
        Err(e) => {
            return Err(AppError::Upstream(format!(
                "node '{}' is unavailable: {}",
                selected.name(), e
            )));
        }
    };

    let status = upstream_resp.status();

    let mut response_builder = HttpResponse::build(status);

    if let Some(content_type) = upstream_resp.headers().get(header::CONTENT_TYPE) {
        response_builder.insert_header((header::CONTENT_TYPE, content_type.clone()));
    }

    let payload = match upstream_resp.body().limit(MAX_UPSTREAM_BODY).await {
        Ok(p) => p,
        Err(e) => {
            return Err(AppError::Upstream(format!(
                "node '{}' is unavailable: {}",
                selected.name(), e
            )));
        }
    };

    Ok(response_builder.body(payload))
}

#[derive(Debug, Deserialize)]
struct ProxyQuery {
    #[serde(alias = "server")]
    validator: Option<String>,
    #[serde(alias = "region")]
    location: Option<String>,
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: &'static str,
}

#[derive(Debug, Serialize)]
struct ValidatorsResponse {
    validators: Vec<ValidatorSummary>,
}
