use console::Term;
use math::Vector2;

pub mod math;

#[allow(dead_code)]
pub struct Screen {
    title: String,
    size: Vector2<i32>,
    term: Term,
    elements: Vec<Box<dyn Drawable>>,
}


impl Screen {
    pub fn new(title: String, size: Vector2<i32>) -> Screen {
        Screen {
            title, 
            size, 
            term: Term::stdout(), 
            elements: Vec::new(),
        }
    }
}

pub trait Drawable {
    fn draw(&self, destination:Term);
}
