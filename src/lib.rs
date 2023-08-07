use console::{Term, Color, style};

pub mod math;

#[allow(dead_code)]
pub struct Screen {
    title: String,
    size: (u16, u16),
    term: Term,
    elements: Vec<Box<dyn Drawable>>,
}


impl Screen {
    pub fn new(title: String) -> Screen {
        let term = Term::stdout();
        term.set_title(title.clone());
        Screen {
            title, 
            size: term.size(), 
            term, 
            elements: Vec::new(),
        }
    }

    // clear the terminal with the background color specified
    pub fn clear(&mut self, bg_color: &Color) {
        self.size = self.term.size();
        let mut reset_line: String;

        if let Err(_) = self.term.move_cursor_to(0,0) {

        }

        for y in 0..=self.size.0 {
            reset_line = format!("{empty_char: >width$}",
                empty_char = style(y).bg(*bg_color),
                width = (self.size.1) as usize
            );

            print!("{reset_line}");
            if let Err(_) = self.term.move_cursor_to(0, y.into()) {
                
            }
        }

    }

    pub fn draw(&self) {
        todo!();
    }

}

pub trait Drawable {
    fn draw(&self, destination:Term);
}
