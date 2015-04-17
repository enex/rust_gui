#![crate_name = "rui"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![feature(box_syntax, core, libc, collections)]

/*!
[repository](https://github.com/enex/rust_gui)
*/

extern crate libc;
extern crate glutin;
extern crate cairo;
extern crate gl;

pub use glutin::{Event, ElementState, MouseCursor, MouseButton, VirtualKeyCode, Api, WindowBuilder};
pub use context::Context;
use state::State;
use std::default::Default;

pub mod context;
#[macro_use]
pub mod components;
pub mod draw;
pub mod primitives;
pub mod cairo_backend;
pub mod debug;
mod state;

#[macro_use]
mod macros;

pub type ID = [u16;12];

//TODO: maybe change ID type
//TODO: use an array like [u8; 16] for storing keys encoded like 0x[one or two byes][the rest]

#[derive(Debug, Copy, Clone, Default)]
pub struct Color{
	r: f32,
	g: f32,
	b: f32,
	a: f32,
}
impl Color{
	fn rgb(r: u8, g: u8, b: u8) -> Color{
		Color{
			r:((r as f32)/255.),
			g:((g as f32)/255.),
			b:((b as f32)/255.),
			..Default::default()
		}
	}
}

/// Backend which should be implemented to support drawing operations
/// the first backend will be cairo + OpenGL but other backends should follow
pub trait Backend{
	/// load a font form a given path and make it available for later use
	/// if it is called with the same font more than one time nothing
	/// should happen
	fn load_font(&mut self, &str);

	fn draw_path(&mut self, primitives::Path);

	//drawing primitives
	fn draw_line(&mut self, line: primitives::Line){
		let mut p = primitives::Path::new();

		p.move_to(line.x1, line.y1);
		p.line_to(line.x2, line.y2);

		self.draw_path(p);
	}
	fn draw_rect(&mut self, rect: primitives::Rect){
		let mut p = primitives::Path::new();

		p.move_to(rect.x, rect.y);
		p.line_to(rect.x + rect.width, rect.y);
		p.line_to(rect.x + rect.width, rect.y + rect.height);
		p.line_to(rect.x, rect.y + rect.height);
		p.line_to(rect.x, rect.y);

		self.draw_path(p);
	}
	fn draw_circle(&mut self, primitives::Circle){
		//TODO: implement it with draw_path
		unimplemented!()
	}
	fn draw_polygon(&mut self, pg: primitives::Polygon){
		let mut p = primitives::Path::new();
		let mut first = true;

		for c in pg.cords.iter(){
			if first{
				p.move_to(c.0, c.1);
				first = false;
			}else{
				p.line_to(c.0, c.1);
			}
		}

		p.close_path();
		self.draw_path(p);
	}
}

/// the trait implemented by all widgets displayed.
/// A widget can eather have state, in which case it gets its own frame-buffer
/// or it has no state in which case its result will be thrown away after changes.
pub trait Widget{
	/// The state of this widget if the widget has no state, nothing has
	/// to b done, the state type has to implement the default trait and
	/// it will have the dafault value when accessed first.
	type State = ();

	/// the event the widget returns this is a enum most of the times
	type Event = ();

	/// In this function only rendering to the screen and atouching event listeners is done
	/// the state of the component gets passed as a imutable reference, so this rutine is not
	/// able to change anything.
	/// It returns the (width, hight) of the area affected by the render method
	fn render(&self, ctx: &mut Context<Self::Event, Self::State>);

	/// Method which is used by the layout engine to get the size of a component
	/// by default the size will be calculated by using the render function with
	/// blocked drawing, but if speed matters this function can provide faster information
	#[inline(always)]
	fn size(&self) -> (f64, f64) {
		(0.0,0.0)
	}

	/// This function is called on every type of element visible one time
	/// by default it does nothing
	fn init(){}
}

/// This has to be implemented by the root component of an application. This is
/// the only component which holds its state by default and is able to mutade
/// its propreties.
pub trait App{
	fn render<State>(&mut self, ctx: &mut Context<(),State>);
	fn close(&mut self){}
	fn start(&mut self){}
}

/// evaluate the expression, then check for GL error.
macro_rules! glcheck {
	($e: expr) => (
		{
			$e;
			assert_eq!(unsafe {gl::GetError()}, 0);
		}
	)
}

/// make a new graphical interface and draw it
pub fn show<F>(window: &mut glutin::Window, draw: F) where F: Fn(&mut Context<(),()>) {
	let (width, height) = window.get_inner_size().unwrap();
	let (mut width, mut height) = (width as i32, height as i32);

	let mut mouse_pos: (i32, i32) = (0, 0);

	gl::load_with(|symbol| window.get_proc_address(symbol));

	let mut redraw = true;

	let mut state = State::new();//the application state

	// create a cairo surface, this is mutable bacause it has to be resized when the
	// window size changes
	let mut surface = cairo::surface::Surface::create_similar_image(
		cairo::surface::format::Format::ARGB32,
		width,
		height,
	);
	assert_eq!(surface.status(), cairo::Status::Success);
	let mut ctx = cairo::Cairo::create(&mut surface);

	while !window.is_closed() {
		window.wait_events();

		for event in window.poll_events() {
			use glutin::Event::*;
			match event{
				Event::Resized(w, h) => {//handle resizes
					println!("({},{})", w, h);
					width = w as i32;
					height = h as i32;
					redraw = true;
				},
				Event::MouseMoved(p) => {
					mouse_pos = p;
				},
				Event::MouseInput(glutin::ElementState::Pressed, _) => {
					let x = mouse_pos.0;
					let y = mouse_pos.1;
					println!("new data at ({}, {})", x, y);
				},
				_ => ()
			}
			println!("{:?}", event);
		}

		if redraw{
			//render cairo
			ctx.save();
			ctx.set_source_rgba(0.0,0.0,0.25,1.0);
			ctx.paint();

			ctx.move_to(0.0,0.0);
			ctx.line_to(0.5, 0.1);

			unsafe{
				
			}
			println!("draw ({}, {})", width, height);
			redraw = false;
			window.swap_buffers();
		}
	}

	//TODO: clear resources before exiting
}
