use std::{collections::BTreeMap, env, net::SocketAddr, thread, time::Duration};

use axum::{
    extract::Query,
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
use yeetlight::{Brightness, Bulb, Color, Effect, Percentage, Temperature};

#[derive(Debug, Deserialize)]
struct PowerParams {
    bulb: String,
}

async fn handler_power_on(
    Query(params): Query<PowerParams>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let bulb = Bulb::new(&params.bulb);
    let response = bulb
        .set_power(true, Effect::Smooth(500))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(response))
}
async fn handler_power_off(
    Query(params): Query<PowerParams>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let bulb = Bulb::new(&params.bulb);
    let response = bulb
        .set_power(false, Effect::Smooth(500))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(response))
}
async fn handler_power_toggle(
    Query(params): Query<PowerParams>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let bulb = Bulb::new(&params.bulb);
    let props_response = bulb
        .get_props(&["power"])
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let power_state = props_response
        .get(0)
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

async fn handler_morning_alarm(Query(params): Query<PowerParams>) -> StatusCode {
    let bulb = Bulb::new(&params.bulb);
    thread::spawn(move || -> anyhow::Result<()> {
        bulb.set_power(true, Effect::Smooth(500))?;
        bulb.set_brightness(Brightness::new(1)?, Effect::Sudden)?;
        bulb.set_temperature(Temperature::new(6500)?, Effect::Sudden)?;
        for _ in 0..50 {
            if bulb.get_props(&["power"])?[0].as_str() != "on" {
                break;
            }
            bulb.adjust_brightness(Percentage::new(2)?, 60_000)?;
            thread::sleep(Duration::from_secs(60));
        }
        Ok(())
    });
    StatusCode::ACCEPTED
}

#[derive(Debug, Deserialize)]
struct BrightnessParams {
    bulb: String,
    brightness: u16,
}
async fn handler_brightness(
    Query(params): Query<BrightnessParams>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let bulb = Bulb::new(&params.bulb);
    let brightness = Brightness::new(params.brightness)
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;
    let response = bulb
        .set_brightness(brightness, Effect::Smooth(500))
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
    let bulb = Bulb::new(&params.bulb);
    let temperature = Temperature::new(params.temperature)
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;
    let response = bulb
        .set_temperature(temperature, Effect::Smooth(500))
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
    let bulb = Bulb::new(&params.bulb);
    let color = Color::from_hex(&params.color)
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;
    let response = bulb
        .set_color(color, Effect::Smooth(500))
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
    let bulb = Bulb::new(&params.bulb);
    let response: BTreeMap<&str, String> = bulb
        .get_props_map(&["power", "bright", "ct", "rgb", "color_mode"])
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(json!(response)))
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

    let serve_assets = ServeEmbed::<Assets>::new();
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(tracing::Level::INFO))
        .on_response(DefaultOnResponse::new().level(tracing::Level::INFO));
    let routes = Router::new()
        .merge(bulb_v1_routes())
        .nest("/v1", bulb_v1_routes())
        .nest("/v2", bulb_v2_routes())
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
