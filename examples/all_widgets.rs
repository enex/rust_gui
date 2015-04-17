extern crate rui;
extern crate glutin;

use rui::components::*;
//use rui::Font;

fn main(){
	let mut window = glutin::WindowBuilder::new()
		.with_title("rui all_widgets demo".to_string())
		.with_gl_version((3,2))
		.with_gl_debug_flag(true)
		.build().unwrap();

	unsafe { window.make_current() };//make it active

	rui::show(&mut window, |c|{
		//c.add(1, Label::new("Hallo Welt").font_size(120.));

		c.add(2, &Icon::new(fa::android));

		let i = Icon{
			icon: fa::android,
			..c.default()
		};
		c.add(3, &i)
	})
}
