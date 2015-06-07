extern crate rui;
extern crate rui_azure;

use rui::prelude::*;

fn main() {
	let window = glutin::WindowBuilder::new()
		.with_title("todo example".to_string())
		.with_dimensions(500, 500)
		.with_gl(glutin::GlRequest::GlThenGles {
	        opengl_version: (3, 2),
	        opengles_version: (3, 2),
	    })
		.with_gl_debug_flag(true)
		.build().unwrap();

	unsafe { window.make_current() };//make it active

	/*let mut app = App::new(window, TodoApp)
	
	app.load_font("sans", "res/Roboto-Regular.ttf").unwrap();
	app.load_font("font-awesome", "res/fontawesome-webfont.ttf").unwrap();
	app.load_font("sans-bold", "res/Roboto-Bold.ttf").unwrap();
	
	app.show(&mut state);*/
}
