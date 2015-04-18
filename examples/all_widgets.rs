#[macro_use] extern crate rui;
extern crate glutin;

use rui::prelude::*;

fn main(){
	let mut window = glutin::WindowBuilder::new()
		.with_title("rui all_widgets demo".to_string())
		.with_gl_version((3,2))
		.with_gl_debug_flag(true)
		.build().unwrap();

	unsafe { window.make_current() };//make it active

	rui::show(&mut window, |c|{
		c.draw_path(path!(M:10,10; L:200,200; L:300,200; L:500,400; Z:));

		let b = components::Button{
			text: "test-button",
			..c.default()
		};
		c.add(1, b);

		/*let i = Icon{
			icon: fa::android,
			..c.default()
		};
		c.add(3, &i)*/
	})
}
