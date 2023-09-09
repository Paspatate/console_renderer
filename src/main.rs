use console_renderer::{math, Screen, Drawable};
use console_renderer::shape::Line;
//use console::Term;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let test_vec2 = math::Vector2::new(1f64, 5f64);
    let target_vec2 = math::Vector2::new(2f64, 10f64);
    
    let test_vec2 = test_vec2 * 2f64;
    assert_eq!(test_vec2.x, target_vec2.x);

    println!("{:?}", test_vec2);

    let mut screen = Screen::new(String::from("Bonjour"));
 
    let t_line = Line::new(math::Vector2::new(10, 17), math::Vector2::new(50, 4), '#');

    loop {
        screen.clear(None);
        t_line.draw(&screen);
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
