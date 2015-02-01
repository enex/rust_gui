#![crate_name = "gui"]
#![unstable]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

extern crate libc;
extern crate cairo;
extern crate sdl2;

pub use sdl2::event::{Event, EventType};//reexprot events
pub use window::Window;
pub use context::Context;
pub use id::ID;

pub mod context;
pub mod components;
mod id;
mod window;
mod state;

//TODO: use an array like [u8; 16] for storing keys encoded like 0x[one or two byes][the rest]


/// the trait implemented by all widgets displayed.
/// A widget can eather have state, in which case it gets its own frame-buffer
/// or it has no state in which case its result will be thrown away after changes.
pub trait Widget{
	///event type the widget returns
	//type Event;

    /// In this function only rendering to the screen and atouching event listeners is done
    /// the state of the component gets passed as a imutable reference, so this rutine is not
    /// able to change anything.
    /// It returns the (width, hight) of the area affected by the render method
    fn render(&self, ctx: &mut Context);

    /// Method which is used by the layout engine to get the size of a component
    /// by default the size will be calculated by using the render function with
    /// blocked drawing, but if speed matters this function can provide faster information
    #[inline(always)]
    fn size(&self) -> (f64, f64) {
        (0.0,0.0)
    }
}

/// A trait which should be implemented by every Widget featuring child widgets
/// like layout or Tabs or somthing like this.
pub trait Children{
	/// builder pattern like function which makes defining child nodes possible
	fn children() -> Self;
}

/// This has to be implemented by the root component of an application. This is
/// the only component which holds its state by default and is able to mutade
/// its propreties.
pub trait App{
	fn render(&mut self, ctx: &mut Context);
	fn close(&mut self){}
	fn start(&mut self){}
}

//these traits are there to manipulate the state

pub trait ImmutableState{
	fn get_state();
}
pub trait MutState: ImmutableState{
	fn set_state();
}
