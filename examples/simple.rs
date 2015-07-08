/*extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate glium_graphics;
extern crate time;

use std::cell::RefCell;
use std::rc::Rc;
use std::path::Path;
use piston::window::{ WindowSettings, Size };
use piston::event::*;
use glium_graphics::{ GliumGraphics, Glium2d, GliumWindow, GlyphCache };
use glutin_window::{ GlutinWindow, OpenGL };

/// measure the time it takes to evaluate the block content
macro_rules! measure(($name:expr, $b:block )=>({
	let start = time::PreciseTime::now();
	let v = $b;
	let end = time::PreciseTime::now();
	println!("{} took: {} ms",
		$name,
		start.to(end).num_milliseconds()
	);
	v
}));

fn main() {
    let opengl = OpenGL::_3_2;
    let size = Size { width: 800, height: 800 };
    let ref window = Rc::new(RefCell::new(GlutinWindow::new(
        opengl,
        WindowSettings::new("simple rui example", size)
            .exit_on_esc(true)
    )));

    let ref glium_window = GliumWindow::new(window).unwrap();
    let mut glyph_cache = GlyphCache::new(
        Path::new("res/Roboto-Regular.ttf"),
        glium_window.clone()
    ).unwrap();

    let mut g2d = Glium2d::new(opengl, glium_window);

    for _ in window.events().swap_buffers(false)
        .filter_map(|event| event.render_args())
	{
		let mut target = glium_window.draw();
		{
            use graphics::*;
            let mut g = GliumGraphics::new(&mut g2d, &mut target);
            let transform =
                graphics::math::abs_transform(size.width as f64, size.height as f64)
                .trans(10.0, 100.0);
			clear([1.0; 4], &mut g);

			measure!("text", {
			text::Text::colored([0.0, 0.5, 0.0, 1.0], 32).draw(
                "My GUI",
                &mut glyph_cache,
                &default_draw_state(),
                transform,
                &mut g
            );
			});

			line::Line::new([0.0, 0.2, 0.2, 1.0], 1.0)
				.draw([1.0,1.0, 200.0,400.0], &default_draw_state(), transform, &mut g);

			rectangle::Rectangle::new([0.2, 0.8, 0.4, 1.0])
				.shape(rectangle::Shape::Round(7.0, 10))
				.border(rectangle::Border{
					color: [0.0,0.0,0.0,1.0],
					radius: 1.0
				})
				.draw([100.0,40.0, 300.0, 200.0], &default_draw_state(), transform, &mut g);
        }
        measure!("target.finish()", { target.finish() });
    }
}*/
fn main(){
	println!("Hallo Welt");
}
