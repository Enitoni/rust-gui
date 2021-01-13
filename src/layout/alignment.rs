use crate::Float;

#[derive(Clone, Copy, Debug)]
pub enum AlignUnit {
    Start,
    Middle,
    End,
}

impl AlignUnit {
    pub fn index(&self) -> i32 {
        match self {
            AlignUnit::Start => 0,
            AlignUnit::End => 1,
            AlignUnit::Middle => 2,
        }
    }

    pub fn calculate(&self, content: Float, bounds: Float) -> Float {
        match self {
            AlignUnit::Start => 0.,
            AlignUnit::Middle => (bounds / 2.) - content / 2.,
            AlignUnit::End => bounds - content,
        }
    }
}

#[derive(Debug)]
pub struct Alignment {
    horizontal: AlignUnit,
    vertical: AlignUnit,
}

impl Alignment {
    pub fn new(horizontal: AlignUnit, vertical: AlignUnit) -> Alignment {
        Alignment {
            horizontal,
            vertical,
        }
    }

    pub fn as_tuple(&self) -> (AlignUnit, AlignUnit) {
        (self.horizontal, self.vertical)
    }
}
