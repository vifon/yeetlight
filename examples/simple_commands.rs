use tokio::time::{sleep, Duration};

use yeetlight::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    simple_logger::init().unwrap();

    let b = Bulb::new("192.168.2.162".parse()?);
    let mut c = b.connect().await?;
    c.set_power(true, Effect::Smooth(500)).await?;
    sleep(Duration::from_secs(1)).await;
    c.set_brightness(Brightness::new(30)?, Effect::Smooth(500))
        .await?;
    sleep(Duration::from_secs(1)).await;
    c.adjust_brightness(Percentage::new(30)?, 500).await?;
    sleep(Duration::from_secs(1)).await;
    c.set_brightness(Brightness::new(100)?, Effect::Smooth(500))
        .await?;
    c.set_temperature(Temperature::new(4700)?, Effect::Smooth(500))
        .await?;
    // c.set_color(Color::from_hex("FF0000")?, Effect::Smooth(500))?;

    let resp = c
        .get_props_map(&["power", "bright", "ct", "rgb", "color_mode"])
        .await?;
    println!("Props: {:?}", resp);

    Ok(())
}
