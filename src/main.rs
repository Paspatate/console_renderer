use console_renderer::math;
use console::Term;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let test_vec2 = math::Vector2::new(1f64, 5f64);
    let target_vec2 = math::Vector2::new(2f64, 10f64);
    
    let test_vec2 = test_vec2 * 2f64;
    assert_eq!(test_vec2.x, target_vec2.x);

    println!("{:?}", test_vec2);
    
    println!("This is {} neat", console::style("quite").cyan());

    let term: Term = Term::stdout();
    term.clear_screen();
    loop {
        term.write_line(&format!("{:?}", term.size())[..])?;
        term.move_cursor_to(0, 0)?;
    }

    Ok(())
}
