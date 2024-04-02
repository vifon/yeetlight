use yeetlight::bulb::*;

fn main() -> std::io::Result<()> {
    simple_logger::init().unwrap();

    let b = Bulb::new("192.168.2.162");
    b.set_power(true, BulbEffect::Smooth(500))?;

    Ok(())
}
