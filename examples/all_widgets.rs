#[macro_use] extern crate rui;
extern crate glutin;

use rui::prelude::*;

#[derive(Default)]
struct MyApp;
impl Widget for MyApp{
	type Event = ();
	type State = ();

	fn render<C:Context<TWidget=MyApp>>(&self, c: &mut C, _: &()){
		c.translate(200.,100.,);
		c.draw_path(path!(M:10,10; L:200,200; L:300,200; L:500,400; Z:));
		c.reset();
		let b = components::Button{
			text: "test-button",
			height: 30.,
			width: 120.,
			..c.default()
		};
		c.translate(10.,20.);
		c.awe(1, &b, |e,_| println!("Event {:?}", e));
		c.reset();

		let i = components::Icon{
			icon: components::fa::android,
			..c.default()
		};
		c.translate(300.,20.);
		c.add(3, &i)
	}
}

fn main(){
	let window = glutin::WindowBuilder::new()
		.with_title("rui all_widgets demo".to_string())
		.with_gl(glutin::GlRequest::GlThenGles {
	        opengl_version: (3, 2),
	        opengles_version: (3, 2),
	    },)
		.with_gl_debug_flag(true)
		.build().unwrap();

	unsafe { window.make_current() };//make it active

	let mut app = App::new(window, MyApp);
	app.load_font("sans", "res/Roboto-Regular.ttf").unwrap();
	app.load_font("font-awesome", "res/fontawesome-webfont.ttf").unwrap();
	app.load_font("sans-bold", "res/Roboto-Bold.ttf").unwrap();
	app.show(&mut ());
}
