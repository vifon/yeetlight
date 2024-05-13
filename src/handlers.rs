use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{collections::BTreeMap, str::FromStr};

use axum::extract::State;
use axum::{extract::Query, http::StatusCode, response::Json};
use futures::future::TryFutureExt;
use log::{info, warn};
use serde::Deserialize;
use serde_json::{json, Value};
use tokio::task::AbortHandle;

use yeetlight::*;

#[derive(Debug, Default, Clone)]
pub struct AppState {
    saved_state: Arc<Mutex<Option<(Brightness, Temperature)>>>,
    alarm_task: Arc<Mutex<Option<AbortHandle>>>,
}

impl AppState {
    #[allow(dead_code)]
    fn get_saved_state(&self) -> Option<(Brightness, Temperature)> {
        *self.saved_state.lock().unwrap()
    }
    fn set_saved_state(&self, value: Option<(Brightness, Temperature)>) {
        *self.saved_state.lock().unwrap() = value;
    }

    fn swap_saved_state(
        &mut self,
        value: Option<(Brightness, Temperature)>,
    ) -> Option<(Brightness, Temperature)> {
        let mut state = self.saved_state.lock().unwrap();
        let old_state = state.clone();
        *state = value;
        old_state
    }

    fn abort_alarm_task(&mut self) {
        self.replace_alarm_task(None)
    }
    fn replace_alarm_task(&mut self, handle: Option<AbortHandle>) {
        let mut lock = self.alarm_task.lock().unwrap();
        if let Some(ref handle) = *lock {
            handle.abort();
        };
        info!("Replacing the alarm task with: {handle:?}");
        *lock = handle;
    }
}

#[derive(Debug, Deserialize)]
pub struct PowerParams {
    bulb: String,
}

pub async fn power_on(
    Query(params): Query<PowerParams>,
) -> Result<Json<Response>, (StatusCode, String)> {
    let bulb = Bulb::from_str(&params.bulb)
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;
    let mut connection = bulb
        .connect()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let response = connection
        .set_power(true, Effect::Smooth(500))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(response))
}
pub async fn power_off(
    Query(params): Query<PowerParams>,
    State(mut state): State<AppState>,
) -> Result<Json<Response>, (StatusCode, String)> {
    let bulb = Bulb::from_str(&params.bulb)
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;
    let mut connection = bulb
        .connect()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    state.abort_alarm_task();

    if let Some((brightness, temperature)) = state.swap_saved_state(None) {
        let _ = connection
            .set_brightness(brightness, Effect::Smooth(500))
            .await
            .map_err(|e| {
                warn!("Failed to restore brightness: {e}");
                e
            });
        let _ = connection
            .set_temperature(temperature, Effect::Smooth(500))
            .await
            .map_err(|e| {
                warn!("Failed to restore temperature: {e}");
                e
            });
    };

    let response = connection
        .set_power(false, Effect::Smooth(500))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(response))
}
pub async fn power_toggle(
    Query(params): Query<PowerParams>,
    State(state): State<AppState>,
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
        "on" => power_off(Query(params), State(state)).await,
        "off" => power_on(Query(params)).await,
        _ => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unexpected light state: {power_state}"),
        )),
    }
}

pub async fn morning_alarm(
    Query(params): Query<PowerParams>,
    State(mut state): State<AppState>,
) -> Result<StatusCode, (StatusCode, String)> {
    let bulb = Bulb::from_str(&params.bulb)
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;
    let mut connection = bulb
        .connect()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let state_clone = state.clone();
    let timer = async move {
        connection.set_power(true, Effect::Smooth(500)).await?;

        let props = connection.get_props(&["bright", "ct"]).await?;
        let brightness: Brightness = props[0].parse::<u16>().unwrap().into();
        let temperature: Temperature = props[1].parse::<u16>().unwrap().into();
        state_clone.set_saved_state(Some((brightness, temperature)));

        connection
            .set_brightness(Brightness::new(Brightness::MIN)?, Effect::Sudden)
            .await?;
        connection
            .set_temperature(Temperature::new(Temperature::MAX)?, Effect::Sudden)
            .await?;
        for _ in 0..50 {
            if connection.get_props(&["power"]).await?[0].as_str() != "on" {
                anyhow::bail!("Bulb turned off early");
            }
            let duration = 60_000;
            connection
                .adjust_brightness(Percentage::new(2)?, duration)
                .await?;
            tokio::time::sleep(Duration::from_millis(duration as u64)).await;
        }
        Ok::<(), anyhow::Error>(())
    }
    .or_else(|e| async {
        warn!("Timer aborted: {e}");
        anyhow::Result::Err(e)
    });

    let handle = tokio::task::spawn(timer);
    state.replace_alarm_task(Some(handle.abort_handle()));

    Ok(StatusCode::ACCEPTED)
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
