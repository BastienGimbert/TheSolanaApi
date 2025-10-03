use std::net::TcpListener;

use actix_web::{self, App, HttpResponse, HttpServer, web};
use serde_json::json;
use url::Url;

use the_solana_api::{AppState, Validator, ValidatorRegistry, routes};

#[actix_web::test]
async fn forwards_json_rpc_payloads() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind upstream listener");
    let upstream_address = listener.local_addr().expect("upstream addr");

    let server = HttpServer::new(|| {
        App::new().route(
            "/",
            web::post().to(|body: web::Bytes| async move { HttpResponse::Ok().body(body) }),
        )
    })
    .listen(listener)
    .expect("listen")
    .run();

    let server_handle = tokio::spawn(server);

    let upstream_url = format!("http://{}/", upstream_address);
    let validator = Validator::new(
        "upstream-1".into(),
        "lab".into(),
        Url::parse(&upstream_url).unwrap(),
    );
    let registry = ValidatorRegistry::new(vec![validator]).expect("registry");
    let state = AppState::new(registry);

    let app = actix_web::test::init_service(
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(routes::configure),
    )
    .await;

    let payload = json!({ "jsonrpc": "2.0", "id": 1, "method": "getVersion", "params": [] });

    let request = actix_web::test::TestRequest::post()
        .uri("/?server=upstream-1")
        .set_json(&payload)
        .to_request();

    let response = actix_web::test::call_service(&app, request).await;
    assert!(response.status().is_success());

    let body = actix_web::test::read_body(response).await;
    let response_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(response_json, payload);

    server_handle.abort();
}
