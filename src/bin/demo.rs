use yeetlight::*;

fn main() -> anyhow::Result<()> {
    simple_logger::init().unwrap();

    let b = Bulb::new("192.168.2.162");
    b.set_power(true, Effect::Smooth(500))?;
    b.set_brightness(Brightness::new(100)?, Effect::Smooth(500))?;
    b.set_temperature(Temperature::new(4700)?, Effect::Smooth(500))?;

    let resp = b.get_props(&["power", "bright", "ct", "rgb", "color_mode"])?;
    println!("Props: {:?}", resp);

    Ok(())
}
