use std::io::{Error as IoError, ErrorKind};

use actix_web::{App, HttpServer, middleware::Logger, web::Data};
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt};

use the_solana_api::{AppState, Settings, ValidatorRegistry, routes};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_tracing();

    let settings = Settings::from_env().map_err(to_io_error)?;
    let registry =
        ValidatorRegistry::from_csv(settings.validators_csv.as_path()).map_err(to_io_error)?;

    let state = AppState::new(registry);
    let bind_address = settings.bind_address.clone();

    info!(
        %bind_address,
        csv = %settings.validators_csv.display(),
        validators = state.registry().validators().len(),
        "starting server"
    );

    let app_state = state.clone();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(app_state.clone()))
            .configure(routes::configure)
    })
    .bind(bind_address)?
    .run()
    .await
}

fn init_tracing() {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,the_solana_api=info"));

    let _ = fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .compact()
        .try_init();
}

fn to_io_error<E: std::error::Error>(error: E) -> IoError {
    IoError::new(ErrorKind::Other, error.to_string())
}
