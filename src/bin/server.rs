use axum::{
    extract::Query,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use serde_json::json;
use yeetlight::{Brightness, Bulb, Color, Effect, Temperature};

#[derive(Debug, Deserialize)]
struct PowerParams {
    bulb: String,
}

async fn handler_power_on(Query(params): Query<PowerParams>) -> impl IntoResponse {
    let bulb = Bulb::new(&params.bulb);
    let response = bulb.set_power(true, Effect::Smooth(500)).unwrap();
    Json(response)
}
async fn handler_power_off(Query(params): Query<PowerParams>) -> impl IntoResponse {
    let bulb = Bulb::new(&params.bulb);
    let response = bulb.set_power(false, Effect::Smooth(500)).unwrap();
    Json(response)
}
async fn handler_power_toggle(Query(_params): Query<PowerParams>) -> impl IntoResponse {
    todo!();
    // let bulb = Bulb::new(&params.bulb);
    // let response = bulb.set_power(false, Effect::Smooth(500)).unwrap();
    // Json(response)
}

#[derive(Debug, Deserialize)]
struct BrightnessParams {
    bulb: String,
    brightness: u16,
}
async fn handler_brightness(Query(params): Query<BrightnessParams>) -> impl IntoResponse {
    let bulb = Bulb::new(&params.bulb);
    let response = bulb
        .set_brightness(
            Brightness::new(params.brightness).unwrap(),
            Effect::Smooth(500),
        )
        .unwrap();
    Json(response)
}

#[derive(Debug, Deserialize)]
struct TemperatureParams {
    bulb: String,
    temperature: u16,
}
async fn handler_temperature(Query(params): Query<TemperatureParams>) -> impl IntoResponse {
    let bulb = Bulb::new(&params.bulb);
    let response = bulb
        .set_temperature(
            Temperature::new(params.temperature).unwrap(),
            Effect::Smooth(500),
        )
        .unwrap();
    Json(response)
}

#[derive(Debug, Deserialize)]
struct ColorParams {
    bulb: String,
    color: String,
}
async fn handler_color(Query(params): Query<ColorParams>) -> impl IntoResponse {
    let bulb = Bulb::new(&params.bulb);
    let response = bulb
        .set_color(Color::from_hex(&params.color).unwrap(), Effect::Smooth(500))
        .unwrap();
    Json(response)
}

#[derive(Debug, Deserialize)]
struct InfoParams {
    bulb: String,
}

async fn handler_info(Query(params): Query<InfoParams>) -> impl IntoResponse {
    let bulb = Bulb::new(&params.bulb);
    let response = bulb
        .get_props(&["power", "bright", "ct", "rgb", "color_mode"])
        .unwrap();
    Json(json!(response))
}

async fn handler_morning_alarm() -> impl IntoResponse {
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

#[tokio::main]
async fn main() {
    let routes = Router::new()
        .merge(bulb_v1_routes())
        .nest("/v1", bulb_v1_routes())
        .nest("/v2", bulb_v2_routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, routes).await.unwrap();
}
