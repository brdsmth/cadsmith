#[derive(Debug)]
pub struct Cube {
    pub side: f32,
}

impl Cube {
    pub fn new(side: f32) -> Self {
        Self { side }
    }
}