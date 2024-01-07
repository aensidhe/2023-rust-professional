pub mod bulb;
pub mod thermometer;

fn main() {
    println!("Hello, world!");
    let _b = bulb::Bulb::new(3.0_f32);
    let _t = thermometer::Thermometer::default();
}
