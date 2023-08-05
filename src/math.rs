#[derive(Debug)]
pub struct Vector2<T> {
    x: T,
    y: T,
}

impl<T> Vector2<T> {
    pub fn new(x:T, y:T) -> Vector2<T> {
        Vector2 { x, y }
    }
}
