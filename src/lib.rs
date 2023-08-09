use std::{thread, time::Duration};

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
    pub fn clear(&mut self, bg_color: Option<&Color>) {
        self.size = self.term.size();
        let mut reset_line: String;
        let bg_color = match bg_color {
            Some(color) => color,
            None => &Color::Black,
        };

        if let Err(_) = self.term.move_cursor_to(0,0) {

        }

        for y in 0..=self.size.0 {
            reset_line = format!("{empty_char: >width$}",
                empty_char = style("").bg(*bg_color),
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

    pub fn target_fps(fps: i32){
        thread::sleep(Duration::from_secs_f32(1f32/fps as f32));
    }

}

pub trait Drawable {
    fn draw(&self, destination:Term);
}
