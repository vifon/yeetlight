use std::{env, net::SocketAddr};

use axum::{
    extract::State,
    response::Json,
    routing::{get, post},
    Router,
};
use axum_embed::ServeEmbed;
use clap::Parser;
use log::info;
use rust_embed::RustEmbed;
use serde_json::Value;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};

mod handlers;

fn config_routes(config: &Option<Value>) -> Router {
    async fn handler_config(State(config): State<Value>) -> Json<Value> {
        Json(config)
    }

    if let Some(config) = config {
        Router::new()
            .route("/config.json", get(handler_config))
            .with_state(config.clone())
    } else {
        Router::new()
    }
}

fn bulb_v1_routes() -> Router {
    Router::new()
        .route("/on", post(handlers::power_on))
        .route("/off", post(handlers::power_off))
        .route("/toggle", post(handlers::power_toggle))
        .route("/brightness", post(handlers::brightness))
        .route("/temperature", post(handlers::temperature))
        .route("/color", post(handlers::color))
        .route("/info", get(handlers::get_info))
}

fn bulb_v2_routes() -> Router {
    Router::new()
    // .route("/on", get(handlers::power_on))
    // .route("/off", get(handlers::power_off))
}

#[derive(RustEmbed, Clone)]
#[folder = "public/"]
struct Assets;

/// A self-hosted Yeelight smartlight control panel.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Network interface to bind to.
    #[arg(long, default_value = "0.0.0.0:8080")]
    iface: String,

    /// Path to the config.
    #[arg(long, default_value = None)]
    config: Option<String>,

    /// Launch a browser.
    #[arg(long)]
    browse: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    simple_logger::init_with_env()?;

    let args = Args::parse();

    let config = if let Some(config_path) = args.config {
        let config = std::fs::read_to_string(config_path)?;
        Some(serde_json::from_str(config.as_str())?)
    } else {
        None
    };

    let serve_assets = ServeEmbed::<Assets>::new();
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(tracing::Level::INFO))
        .on_response(DefaultOnResponse::new().level(tracing::Level::INFO));
    let routes = Router::new()
        .merge(bulb_v1_routes())
        .nest("/v1", bulb_v1_routes())
        .nest("/v2", bulb_v2_routes())
        .merge(config_routes(&config))
        .fallback_service(serve_assets)
        .layer(trace_layer);

    let bind_addr: SocketAddr = args.iface.parse()?;
    let listener = tokio::net::TcpListener::bind(bind_addr).await?;

    if args.browse {
        std::process::Command::new("xdg-open")
            .arg(format!("http://{bind_addr}"))
            .spawn()
            .expect("Failed to launch the web browser");
    }

    info!("Listening on http://{bind_addr}");
    axum::serve(listener, routes).await?;

    Ok(())
}
