use std::{collections::BTreeMap, str::FromStr};

use axum::{extract::Query, http::StatusCode, response::Json};
use serde::Deserialize;
use serde_json::{json, Value};

use yeetlight::*;

#[derive(Debug, Deserialize)]
pub struct PowerParams {
    bulb: String,
}

pub async fn power_on(
    Query(params): Query<PowerParams>,
) -> Result<Json<Response>, (StatusCode, String)> {
    let bulb = Bulb::from_str(&params.bulb)
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;
    let response = bulb
        .connect()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .set_power(true, Effect::Smooth(500))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(response))
}
pub async fn power_off(
    Query(params): Query<PowerParams>,
) -> Result<Json<Response>, (StatusCode, String)> {
    let bulb = Bulb::from_str(&params.bulb)
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;
    let response = bulb
        .connect()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .set_power(false, Effect::Smooth(500))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(response))
}
pub async fn power_toggle(
    Query(params): Query<PowerParams>,
) -> Result<Json<Response>, (StatusCode, String)> {
    let bulb = Bulb::from_str(&params.bulb)
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;
    let props_response = bulb
        .connect()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .get_props(&["power"])
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let power_state = props_response
        .first()
        .expect("Got a response but with no expected value");
    match power_state.as_str() {
        "on" => power_off(Query(params)).await,
        "off" => power_on(Query(params)).await,
        _ => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unexpected light state: {power_state}"),
        )),
    }
}

#[derive(Debug, Deserialize)]
pub struct BrightnessParams {
    bulb: String,
    brightness: u16,
}
pub async fn brightness(
    Query(params): Query<BrightnessParams>,
) -> Result<Json<Response>, (StatusCode, String)> {
    let bulb = Bulb::from_str(&params.bulb)
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;
    let brightness = Brightness::new(params.brightness)
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;
    let response = bulb
        .connect()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .set_brightness(brightness, Effect::Smooth(500))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(response))
}

#[derive(Debug, Deserialize)]
pub struct TemperatureParams {
    bulb: String,
    temperature: u16,
}
pub async fn temperature(
    Query(params): Query<TemperatureParams>,
) -> Result<Json<Response>, (StatusCode, String)> {
    let bulb = Bulb::from_str(&params.bulb)
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;
    let temperature = Temperature::new(params.temperature)
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;
    let response = bulb
        .connect()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .set_temperature(temperature, Effect::Smooth(500))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(response))
}

#[derive(Debug, Deserialize)]
pub struct ColorParams {
    bulb: String,
    color: String,
}
pub async fn color(
    Query(params): Query<ColorParams>,
) -> Result<Json<Response>, (StatusCode, String)> {
    let bulb = Bulb::from_str(&params.bulb)
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;
    let color = Color::from_hex(&params.color)
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;
    let response = bulb
        .connect()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .set_color(color, Effect::Smooth(500))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(response))
}

#[derive(Debug, Deserialize)]
pub struct InfoParams {
    bulb: String,
}

pub async fn get_info(
    Query(params): Query<InfoParams>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let bulb = Bulb::from_str(&params.bulb)
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;
    let response: BTreeMap<&str, String> = bulb
        .connect()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .get_props_map(&["power", "bright", "ct", "rgb", "color_mode"])
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(json!(response)))
}
