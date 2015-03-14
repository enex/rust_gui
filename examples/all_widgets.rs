extern crate rui;
extern crate glutin;

use rui::components::*;
use rui::Font;
//TODO: insert widgets here

fn rgba(r:u8, g:u8, b:u8, a:u8) -> rui::Color { rui::Color::rgba(r,g,b,a) }

fn main(){
	let mut window = glutin::WindowBuilder::new()
		.with_title("rui all_widgets demo".to_string())
		.with_gl_version((3,2))
		.with_gl_debug_flag(true)
		.build().unwrap();

	unsafe { window.make_current() };//make it active
	
	rui::show(&mut window, |c|{
		c.add(1, Label::new("Hallo Welt").font_size(120.));

		let i = Icon::new(fa::android);
		c.add(2, &i);

		c.draw(|vg|{
			vg.begin_path();
			vg.move_to(0.,0.);
			vg.line_to(100.,0.);
			vg.line_to(100.,100.);
			vg.line_to(0.,100.);
			vg.close_path();
			vg.rect(0.,0., 200.,200.);
			vg.fill_color(rgba(255,50,50,255));
			vg.fill();
		});
		/*{
			use rui::Path::*;
			c.path(&[M(0.,0.),L(100.,0.),L(100.,100.),L(0.,100.),C]).color(23,3,23).stroke();
		}*/
	})
}
