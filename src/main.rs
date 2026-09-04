use actix_web::{
    App, HttpServer,
    middleware::{NormalizePath, TrailingSlash},
    web::Data,
};
use kwi::{
    CONFIG,
    helpers::{AppState, Config, malformed_request_handler},
    route,
};
use tracing::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("kwi=debug,actix_web=info")
        .init();

    let loaded_config = Config::from_env().expect("Could not load config");
    CONFIG.set(loaded_config).unwrap();

    let app_state = AppState::try_init()
        .await
        .unwrap_or_else(|e| panic!("Could not initialize critical connection: {:?}", e));

    //
    // Start server
    //
    let config = CONFIG.get().unwrap();
    let hostname = format!("{}:{}", &config.app_host, &config.app_port);
    info!("Running server on http://{}", hostname);

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .app_data(malformed_request_handler::json())
            .app_data(malformed_request_handler::path())
            .app_data(Data::new(app_state.clone()))
            .configure(route::config)
    })
    .bind(hostname)
    .expect("Unable to bind to host")
    .run()
    .await
}
