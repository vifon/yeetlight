use axum::{
    extract::Query,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use axum_embed::ServeEmbed;
use log::info;
use rust_embed::RustEmbed;
use serde::Deserialize;
use serde_json::{json, Value};
use yeetlight::{Brightness, Bulb, Color, Effect, Temperature};

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
        .get("power")
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
    let response = bulb
        .get_props(&["power", "bright", "ct", "rgb", "color_mode"])
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(json!(response)))
}

async fn handler_morning_alarm() -> Json<Value> {
    todo!()
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    simple_logger::init().unwrap();

    let args: Vec<String> = std::env::args().collect();
    let bind_addr = match args.len() {
        1 => Ok("0.0.0.0:8080"),
        2 => Ok(args[1].as_ref()),
        _ => Err(anyhow::Error::msg(format!("Wrong arguments: {args:?}"))),
    }?;

    let serve_assets = ServeEmbed::<Assets>::new();
    let routes = Router::new()
        .merge(bulb_v1_routes())
        .nest("/v1", bulb_v1_routes())
        .nest("/v2", bulb_v2_routes())
        .fallback_service(serve_assets);

    let listener = tokio::net::TcpListener::bind(bind_addr).await?;
    info!("Listening on http://{bind_addr}");
    axum::serve(listener, routes).await?;

    Ok(())
}
