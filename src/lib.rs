#![crate_name = "rui"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![feature(box_syntax, core, libc, collections)]

/*!
[repository](https://github.com/enex/rust_gui)
*/

extern crate libc;
extern crate glutin;
extern crate gl;
extern crate time;

pub use glutin::{MouseCursor, MouseButton, VirtualKeyCode, Api, WindowBuilder};
pub use context::{Context, DrawContext, EventContext, Common};
pub use color::Color;
pub use transform::Transform;

use std::fmt;
use state::State;
use std::default::Default;
use std::any::Any;
use context::StateT;
use glutin::Event;
use std::intrinsics;
use backend::Backend;

pub mod context;
#[macro_use]
pub mod components;
#[macro_use]
pub mod draw;
pub mod primitives;
pub mod transform;
pub mod color;
//pub mod debug;
mod state;
pub mod backend;

#[macro_use]
mod macros;

pub trait ColorTrait{
	fn rgb(r: u8, g: u8, b: u8) -> Self;
}
impl ColorTrait for Color{
	fn rgb(r: u8, g: u8, b: u8) -> Self{
		Color{
			r: r as f32,
			g: g as f32,
			b: b as f32,
			a: 1.
		}
	}
}

pub mod prelude{
	//! this is a bundle of the things most frequently needed
	//!
	//! 	use rui::prelude::*;
	//!
	//! it also includes glutin

	pub use primitives;
	pub use Widget;
	pub use context::Context;
	pub use components;
	pub use draw::{Path, PathInstr, AsPath};
	pub use context::{EventHandle, StateT};
	pub use App;
	pub use Color;
	pub use glutin;
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ID (pub [u16;12]);
impl fmt::Debug for ID{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
		let mut first = true;
		try!(write!(f,"$"));
		for i in self.0.iter(){
			if *i == 0{
				break;
			}
			if !first{
				try!(write!(f,"-"));
			}
			try!(write!(f, "{}", i));
			first = false;
		}
		Ok(())
	}
}
impl ID{
	pub fn null() -> ID{
		ID([0;12])
	}
	/// whether it is the id of the root node
	pub fn is_root(&self) -> bool{
		self.0[0] == 0
	}
}

//TODO: maybe change ID type
//TODO: use an array like [u8; 16] for storing keys encoded like 0x[one or two byes][the rest]

/// the trait implemented by all widgets displayed.
/// A widget can eather have state, in which case it gets its own frame-buffer
/// or it has no state in which case its result will be thrown away after changes.
pub trait Widget{
	/// The state of this widget if the widget has no state, nothing has
	/// to b done, the state type has to implement the default trait and
	/// it will have the dafault value when accessed first.
	type State: Any + Default = ();

	/// the event the widget returns. This is a enum most of the times.
	type Event = ();

	/// In this function only rendering to the screen and atouching event listeners is done
	/// the state of the component gets passed as a imutable reference, so this rutine is not
	/// able to change anything.
	/// It returns the (width, hight) of the area affected by the render method
	fn render<C:Context<TWidget=Self>>(&self, ctx: &mut C, s: &Self::State);

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
	fn name() -> &'static str{
		unsafe{
			intrinsics::type_name::<Self>()
		}
	}

	/// function to add a widget to the context.
	fn draw<C:Context>(&self, c: &mut C, id: u16) where Self: Widget+Sized{
		c.add(id, self)
	}
}

/// trait which should be implmented by everything used as ui state
pub trait UIState<E>: StateT{
	/// handle a event emited from the root component
	fn handle(&mut self, e: E);
}
impl UIState<()> for (){
	fn handle(&mut self, _:()){}
}
/// trait possibly used for the context to add widgets, could abstract
/// transformation
pub trait Adder<T>{
	fn add(&mut self, u16, &T);
}
pub trait AdderEvent<T>{
	fn awe(&mut self, u16, &T);
}

/// this represents a whole app and contains the root widget
pub struct App<W,D:Backend>{
	/// the window struct responsible for event listening
	window: glutin::Window,
	/// the root node which is a Widget
	root: W,
	/// the size of the window (width, height)
	size: (i32, i32),
	/// whether drawing has already begun
	begun: bool,
	/// whether the frame has to be redrawn
	redraw: bool,
	///the data relevant for the contexts
	data: Common<D>,
}

impl<W:Widget<State=S,Event=E>,S:UIState<E>, E, D:Backend> App<W, D>{
	pub fn new(window: glutin::Window, root:W) -> App<W, D>{
		let (w, h) = match window.get_inner_size(){
			Some(s) => s,
			None => {
				println!("was nat able to get inner sizer");
				(0, 0)
			}
		};
		App{
			root: root,
			size: (w as i32, h as i32),
			data: Common{
				be: {
					gl::load_with(|symbol| window.get_proc_address(symbol));
					D::new(&window)
				},
				state: State::new(),
				depth: 0,
				id: ID::null(),
				transform: Transform::normal(),
				listeners: Vec::new(),
				event: Event::Resized(w, h),
				affected: Vec::new(),
				redraw: Vec::new(),
				mouse_pos: (0.0, 0.0),
			},
			window: window,
			begun: false,
			redraw: true,
		}
	}

	/// load a font which is then available with the specified name. If the font
	/// could not be loaded, an error will be returned.
	pub fn load_font(&mut self, name: &str, path: &str) -> Result<(),()>{
		self.data.be.load_font(name, path)
	}

	/// start drawing process if not alerady started
	fn ps(&mut self){
		self.data.be.begin(self.size.0, self.size.1);
		self.begun = true;
	}

	/// start the application with a reference to the state, this will be passed
	/// to the render function of the root component
	pub fn show(&mut self, state: &mut S){
		while !self.window.is_closed() {
			use glutin::Event::*;

			let event = match self.window.wait_events().next(){
				Some(e) => e,
				None => break
			};

			let start = time::PreciseTime::now();

			match event{
				Resized(w, h) => {//handle resizes
					self.size.0 = w as i32;
					self.size.1 = h as i32;
					self.redraw = true;
					unsafe{//make sure resize does'n result in stretching and displacement
						gl::Viewport(0, 0, self.size.0, self.size.1);
					}
				},
				MouseMoved((x,y)) => {
					//TODO: also use this o listen for events
					self.data.mouse_pos = (x as f32, y as f32);
				},
				MouseInput(..) | ReceivedCharacter(..) | MouseWheel(..) |
				KeyboardInput(_, _, _) => {
					self.data.event = event;
					let mut c:EventContext<D, W> = EventContext::new(&mut self.data);
					self.root.render(&mut c, state);
					for e in c.emited{
						state.handle(e);
						self.redraw = true;
					}
				},
				Refresh => { self.redraw = true; },
				Moved(_, _) => (),
				Closed => (),
				Focused(_) => (),
				Awakened => (),
			}

			let end = time::PreciseTime::now();
			println!("event handling took: {}", start.to(end).num_milliseconds());

			self.data.be.reset_transform();
			if self.redraw{
				let start = time::PreciseTime::now();
				self.ps();
				{
					let mut c:DrawContext<D, W> =
						DrawContext::new(&mut self.data);
					self.root.render(&mut c, state);
				}
				//println!("draw ({}, {})", self.size.0, self.size.1);
				self.redraw = false;

				if self.begun{
					self.data.be.end();
					self.data.be.reset_transform();
					self.window.swap_buffers();
					self.begun = false;
				}
				let end = time::PreciseTime::now();
				println!("drawing took: {} ms", start.to(end).num_milliseconds());
			}
		}
	}
}
