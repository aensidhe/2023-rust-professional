pub type Watts = f32;

pub enum Mode {
    SwitchedOn,
    SwitchedOff,
}

pub struct Bulb {
    power: Watts,
    mode: Mode,
}

impl Bulb {
    pub fn new(power: Watts) -> Self {
        Self {
            power,
            mode: Mode::SwitchedOff,
        }
    }

    pub fn get_mode(&self) -> f32 {
        match self.mode {
            Mode::SwitchedOn => self.power,
            Mode::SwitchedOff => 0_f32,
        }
    }

    pub fn toggle(&mut self) {
        self.mode = match self.mode {
            Mode::SwitchedOn => Mode::SwitchedOff,
            Mode::SwitchedOff => Mode::SwitchedOn,
        };
    }

    pub fn get_description() {
        todo!()
    }
}
