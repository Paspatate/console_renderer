#[derive(Debug)]
pub struct Vector2f {
    x: f64,
    y: f64,
}

impl Vector2f {
    pub fn new(x:f64, y:f64) -> Vector2f {
        Vector2f { x, y }
    }
}