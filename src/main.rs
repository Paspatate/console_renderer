use console_renderer::math::Vector3;
use console_renderer::shape::{Line, Cube};
use console_renderer::{math, Drawable, Screen};
//use console::Term;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_vec2 = math::Vector2::new(1f64, 5f64);
    let target_vec2 = math::Vector2::new(2f64, 10f64);
    let mut t_vec3 = math::Vector3::new(1i32, 1i32, 1i32);
    t_vec3 = t_vec3 * 2;
    let t2_vec3 = math::Vector3::new(2i32, 2i32, 2i32);
    let test_vec2 = test_vec2 * 2f64;
    assert_eq!(test_vec2.x, target_vec2.x);
    assert_eq!(t_vec3.z, t2_vec3.z);

    println!("{:?}", test_vec2);

    let mut screen = Screen::new(String::from("Bonjour"));

    let mut a = 20f32;
    let mut b = 10f32;
    let mut t_line = Line::new(
        math::Vector2::new(10, 10),
        math::Vector2::new(a as i32, b as i32),
        '#',
    );

    let mut t_cube = Cube::new(10, &Vector3::new(-100i32, 20i32, 30i32));

    let mut run = true;
    while run {
        //a = (a.cos() * 10f32) + 20f32;
        //b = (b.sin() * 10f32) + 20f32;
        a -= 1.0;
        b -= 1.0;
        t_line.point2.x = a as i32;
        t_line.point2.y = b as i32;
        //t_cube.translate(1, 0, 0);
        screen.clear(None);
        //t_line.draw(&screen);
        t_cube.draw(&screen);
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
