use std::{thread::sleep, time::Duration};

use yeetlight::*;

fn main() -> anyhow::Result<()> {
    simple_logger::init().unwrap();

    let b = Bulb::new("192.168.2.162");
    b.set_power(true, Effect::Smooth(500))?;
    sleep(Duration::from_secs(1));
    b.set_brightness(Brightness::new(30)?, Effect::Smooth(500))?;
    sleep(Duration::from_secs(1));
    b.adjust_brightness(Percentage::new(30)?, 500)?;
    sleep(Duration::from_secs(1));
    b.set_brightness(Brightness::new(100)?, Effect::Smooth(500))?;
    b.set_temperature(Temperature::new(4700)?, Effect::Smooth(500))?;
    // b.set_color(Color::from_hex("FF0000")?, Effect::Smooth(500))?;

    let resp = b.get_props_map(&["power", "bright", "ct", "rgb", "color_mode"])?;
    println!("Props: {:?}", resp);

    Ok(())
}
