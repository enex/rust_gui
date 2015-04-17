#![crate_name = "rui"]
#![unstable]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![feature(box_syntax, core, libc, collections)]

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
mod state;
pub mod draw;

#[macro_use]
mod macros;

pub type ID = [u16;12];

//TODO: use an array like [u8; 16] for storing keys encoded like 0x[one or two byes][the rest]
//TODO: add android support

#[derive(Debug, Copy, Clone, Default)]
pub struct Color{
	r: f32,
	g: f32,
	b: f32
}
impl Color{
	fn rgb(r: u8, g: u8, b: u8) -> Color{
		Default::default()
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
	fn init(){
		//do nothing
	}
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
	let (mut width, mut height) = window.get_inner_size().unwrap();

	let mut mouse_pos: (i32, i32) = (0, 0);

	unsafe{
		gl::load_with(|symbol| window.get_proc_address(symbol));
		gl::ClearColor(0.1, 0.1, 0.1, 1.0);
	}

	let mut redraw = true;

	let mut state = State::new();//the application state

	while !window.is_closed() {
		window.wait_events();

		for event in window.poll_events() {
			use glutin::Event::*;
			match event{
				Event::Resized(w, h) => {//handle resizes
					println!("({},{})",w,h);
					width = w;
					height = h;
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
			//TODO: do draw here

			println!("draw ({}, {})", width, height);
			redraw = false;
		}
	}
}
