#[derive(Copy, Clone)]
pub enum Effect {
    Smooth(u16),
    Sudden,
}

impl Effect {
    pub fn effect(&self) -> &'static str {
        match self {
            Effect::Smooth(_) => "smooth",
            Effect::Sudden => "sudden",
        }
    }

    pub fn duration(&self) -> u16 {
        match self {
            Effect::Smooth(x) => *x,
            Effect::Sudden => 0,
        }
    }
}
