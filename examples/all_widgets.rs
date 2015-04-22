#[macro_use] extern crate rui;
extern crate glutin;

use rui::prelude::*;

#[derive(Default)]
struct MyApp;
impl Widget for MyApp{
	type Event = ();
	type State = ();

	fn render<C:Context>(&self, c: &mut C){
		c.draw_path(path!(M:10,10; L:200,200; L:300,200; L:500,400; Z:));

		let b = components::Button{
			text: "test-button",
			..c.default()
		};
		c.awe(1, b, |e,_| println!("Event {:?}", e));

		/*let i = Icon{
			icon: fa::android,
			..c.default()
		};
		c.add(3, &i)*/
	}
}

fn main(){
	let window = glutin::WindowBuilder::new()
		.with_title("rui all_widgets demo".to_string())
		.with_gl_version((3,2))
		.with_gl_debug_flag(true)
		.build().unwrap();

	unsafe { window.make_current() };//make it active

	App::new(window, MyApp).show();
}
