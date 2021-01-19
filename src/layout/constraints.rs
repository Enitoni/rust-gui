use super::Float;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum ConstraintUnit {
    Percent(Float),
    Fixed(Float),
    None,
}

impl ConstraintUnit {
    pub fn calculate(&self, value: Float) -> Float {
        match self {
            ConstraintUnit::Percent(p) => value * p,
            ConstraintUnit::Fixed(f) => *f,
            ConstraintUnit::None => value,
        }
    }

    pub fn index(&self) -> u32 {
        match self {
            ConstraintUnit::Percent(_) => 1,
            ConstraintUnit::Fixed(_) => 2,
            ConstraintUnit::None => 3,
        }
    }
}
