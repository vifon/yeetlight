use std::{collections::BTreeMap, env, net::SocketAddr, time::Duration};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use axum_embed::ServeEmbed;
use clap::Parser;
use log::info;
use rust_embed::RustEmbed;
use serde::Deserialize;
use serde_json::{json, Value};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};

use yeetlight::*;

#[derive(Debug, Deserialize)]
struct PowerParams {
    bulb: String,
}

async fn handler_power_on(
    Query(params): Query<PowerParams>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let bulb = Bulb::from_str(&params.bulb)
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;
    let response = bulb
        .set_power(true, Effect::Smooth(500))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(response))
}
async fn handler_power_off(
    Query(params): Query<PowerParams>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let bulb = Bulb::from_str(&params.bulb)
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;
    let response = bulb
        .set_power(false, Effect::Smooth(500))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(response))
}
async fn handler_power_toggle(
    Query(params): Query<PowerParams>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let bulb = Bulb::from_str(&params.bulb)
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;
    let props_response = bulb
        .get_props(&["power"])
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let power_state = props_response
        .first()
        .expect("Got a response but with no expected value");
    match power_state.as_str() {
        "on" => handler_power_off(Query(params)).await,
        "off" => handler_power_on(Query(params)).await,
        _ => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unexpected light state: {power_state}"),
        )),
    }
}

async fn handler_morning_alarm(
    Query(params): Query<PowerParams>,
) -> Result<StatusCode, (StatusCode, String)> {
    let bulb = Bulb::from_str(&params.bulb)
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;
    tokio::task::spawn(async move {
        bulb.set_power(true, Effect::Smooth(500)).await.unwrap();
        bulb.set_brightness(Brightness::new(Brightness::MIN).unwrap(), Effect::Sudden)
            .await
            .unwrap();
        bulb.set_temperature(Temperature::new(Temperature::MAX).unwrap(), Effect::Sudden)
            .await
            .unwrap();
        for _ in 0..50 {
            if bulb.get_props(&["power"]).await.unwrap()[0].as_str() != "on" {
                break;
            }
            let duration = 60_000;
            bulb.adjust_brightness(Percentage::new(2).unwrap(), duration)
                .await
                .unwrap();
            tokio::time::sleep(Duration::from_millis(duration as u64)).await;
        }
    });
    Ok(StatusCode::ACCEPTED)
}

#[derive(Debug, Deserialize)]
struct BrightnessParams {
    bulb: String,
    brightness: u16,
}
async fn handler_brightness(
    Query(params): Query<BrightnessParams>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let bulb = Bulb::from_str(&params.bulb)
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;
    let brightness = Brightness::new(params.brightness)
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;
    let response = bulb
        .set_brightness(brightness, Effect::Smooth(500))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(response))
}

#[derive(Debug, Deserialize)]
struct TemperatureParams {
    bulb: String,
    temperature: u16,
}
async fn handler_temperature(
    Query(params): Query<TemperatureParams>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let bulb = Bulb::from_str(&params.bulb)
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;
    let temperature = Temperature::new(params.temperature)
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;
    let response = bulb
        .set_temperature(temperature, Effect::Smooth(500))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(response))
}

#[derive(Debug, Deserialize)]
struct ColorParams {
    bulb: String,
    color: String,
}
async fn handler_color(
    Query(params): Query<ColorParams>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let bulb = Bulb::from_str(&params.bulb)
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;
    let color = Color::from_hex(&params.color)
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;
    let response = bulb
        .set_color(color, Effect::Smooth(500))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(response))
}

#[derive(Debug, Deserialize)]
struct InfoParams {
    bulb: String,
}

async fn handler_info(
    Query(params): Query<InfoParams>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let bulb = Bulb::from_str(&params.bulb)
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;
    let response: BTreeMap<&str, String> = bulb
        .get_props_map(&["power", "bright", "ct", "rgb", "color_mode"])
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(json!(response)))
}

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
        .route("/on", post(handler_power_on))
        .route("/off", post(handler_power_off))
        .route("/toggle", post(handler_power_toggle))
        .route("/brightness", post(handler_brightness))
        .route("/temperature", post(handler_temperature))
        .route("/color", post(handler_color))
        .route("/info", get(handler_info))
        .route("/alarm", post(handler_morning_alarm))
}

fn bulb_v2_routes() -> Router {
    Router::new()
    // .route("/on", get(handler_power_on))
    // .route("/off", get(handler_power_off))
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
