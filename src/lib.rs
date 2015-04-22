#![crate_name = "rui"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![feature(box_syntax, core, libc, collections)]

/*!
[repository](https://github.com/enex/rust_gui)
*/

extern crate libc;
extern crate glutin;
extern crate nanovg;
extern crate gl;

pub use glutin::{ElementState, MouseCursor, MouseButton, VirtualKeyCode, Api, WindowBuilder};
use glutin::Event;
pub use context::{Context, DrawContext, EventContext, Common};
use state::State;
use std::default::Default;
pub use nanovg::{Ctx, Color, Font};
use nanovg_backend::NanovgBackend;
use draw::{Path, PathInstr};
use std::any::Any;
use context::StateT;

pub mod context;
#[macro_use]
pub mod components;
#[macro_use]
pub mod draw;
pub mod primitives;
pub mod nanovg_backend;
//pub mod debug;
mod state;

#[macro_use]
mod macros;

pub mod prelude{
	//! this is a bundle of the things most frequently needed
	//!
	//! 	use rui::prelude::*;

	pub use primitives;
	pub use Widget;
	pub use context::Context;
	pub use components;
	pub use draw::{Path, PathInstr};
	pub use context::EventHandle;
	pub use App;
}

pub type ID = [u16;12];

//TODO: maybe change ID type
//TODO: use an array like [u8; 16] for storing keys encoded like 0x[one or two byes][the rest]

//TODO: replace path conversions with more performant zero allocation ones

/// Backend which should be implemented to support drawing operations
/// the first backend will be cairo + OpenGL but other backends should follow
pub trait Backend{
	/// load a font form a given path and make it available for later use
	/// if it is called with the same font more than one time nothing
	/// should happen
	fn load_font(&mut self, &str, &str);

	fn begin(&mut self, _:i32, _:i32){}

	fn draw_path<I:AsRef<[draw::PathInstr]>, V:AsRef<[f32]>>
		(&mut self, primitives::Path<I, V>);

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

	fn end(&mut self){}
}

/// the trait implemented by all widgets displayed.
/// A widget can eather have state, in which case it gets its own frame-buffer
/// or it has no state in which case its result will be thrown away after changes.
pub trait Widget{
	/// The state of this widget if the widget has no state, nothing has
	/// to b done, the state type has to implement the default trait and
	/// it will have the dafault value when accessed first.
	type State: Any + Default;

	/// the event the widget returns this is a enum most of the times
	type Event;

	/// In this function only rendering to the screen and atouching event listeners is done
	/// the state of the component gets passed as a imutable reference, so this rutine is not
	/// able to change anything.
	/// It returns the (width, hight) of the area affected by the render method
	fn render<C:Context<TWidget=Self>>(&self, ctx: &mut C);

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

	/// the name of the widget, this is optional and for debuging
	fn name() -> &'static str{""}
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

/// this represents a whole app and contains the root wiget
pub struct App<W,D:Backend>{
	window: glutin::Window,
	root: W,
	size: (i32, i32),
	begun: bool,
	redraw: bool,
	data: Common<D>,
}

impl<W:Widget<State=S,Event=E>,S:StateT,E> App<W, NanovgBackend>{
	pub fn new(window: glutin::Window, root:W) -> App<W, NanovgBackend>{
		App{
			root: root,
			size: {
				let (w, h) = window.get_inner_size().unwrap();
				(w as i32, h as i32)
			},
			data: Common{
				be: {
					gl::load_with(|symbol| window.get_proc_address(symbol));
					let vg = Ctx::create_gl3(nanovg::ANTIALIAS | nanovg::STENCIL_STROKES);
					NanovgBackend::new(vg)
				},
				state: State::new(),
				depth: 0,
				id: [0; 12],
			},
			window: window,
			begun: false,
			redraw: true,
		}
	}

	/// start drawing process if not alerady started
	fn ps(&mut self){
		self.data.be.begin(self.size.0, self.size.1);
		self.begun = true;
	}

	/// start the application
	pub fn show(&mut self){
		self.data.be.load_font("sans", "res/Roboto-Regular.ttf");
		self.data.be.load_font("font-awesome", "res/fontawesome-webfont.ttf");
		self.data.be.load_font("sans-bold", "res/Roboto-Bold.ttf");

		while !self.window.is_closed() {
			self.window.wait_events();

			for event in self.window.poll_events() {
				use glutin::Event::*;

				match event{
					Event::Resized(w, h) => {//handle resizes
						self.size.0 = w as i32;
						self.size.1 = h as i32;
						self.redraw = true;
					},
					Event::MouseInput(glutin::ElementState::Pressed, _) => {
						//let x = mouse_pos.0;
						//let y = mouse_pos.1;
						//TODO: implement event handling
						println!("mouse input");
						let mut c:EventContext<NanovgBackend,W> = EventContext::new(&mut self.data);
						self.root.render(&mut c);
					},
					_ => ()
				}
				//println!("{:?}", event);
			}

			if self.redraw{
				self.ps();
				{
					let mut c:DrawContext<NanovgBackend, W> =
						DrawContext::new(&mut self.data);
					self.root.render(&mut c);
				}
				println!("draw ({}, {})", self.size.0, self.size.1);
				self.redraw = false;
			}

			if self.begun{
				self.data.be.end();
				self.window.swap_buffers();
				self.begun = false;
			}
		}
	}
}
