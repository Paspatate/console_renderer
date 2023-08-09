use console_renderer::{math, Screen};
use std::{thread, time::Duration};
//use console::Term;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let test_vec2 = math::Vector2::new(1f64, 5f64);
    let target_vec2 = math::Vector2::new(2f64, 10f64);
    
    let test_vec2 = test_vec2 * 2f64;
    assert_eq!(test_vec2.x, target_vec2.x);

    println!("{:?}", test_vec2);

    let mut screen = Screen::new(String::from("Bonjour"));

    let mut bg;
    let mut count:u64 = 0;
    loop {
        if count%2 == 1 {
            bg = Some(&console::Color::Blue);    
        } else {
            bg = None;
        }
        screen.clear(bg);
        count += 1;

        Screen::target_fps(30);
    }
    
//    println!("This is {} neat", console::style("quite").cyan());
//
//    let style_t = console::Style::new();
//
//    println!("le fond est {}", style_t.bg(console::Color::Green).apply_to("Vert"));
//
//    let term: Term = Term::stdout();
//    
//    loop {
//        term.write_line(&format!("{:?}", term.size())[..])?;
//        term.move_cursor_to(0, 0)?;
//        
//    }
//
    Ok(())
}
