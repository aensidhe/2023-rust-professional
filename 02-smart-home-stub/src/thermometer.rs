pub type Celcius = f32;

#[derive(Default)]
pub struct Thermometer {
    last_reading: Celcius,
}

impl Thermometer {
    pub fn get_reading(&self) -> Celcius {
        self.last_reading
    }
}
