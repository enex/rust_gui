#![crate_name = "gui"]
#![unstable]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

extern crate libc;
extern crate cairo;
extern crate sdl2;

pub use sdl2::event::Event;//reexprot events
pub use window::Window;

pub mod components;
mod window;
mod ctx;

/// the trait implemented by all widgets displayed.
/// A widget can eather have state, in which case it gets its own frame-buffer
/// or it has no state in which case its result will be thrown away after changes.
pub trait Widget{
	///event type the widget returns
	type Event;

    /// In this function only rendering to the screen and atouching event listeners is done
    /// the state of the component gets passed as a imutable reference, so this rutine is not
    /// able to change anything.
    /// It returns the (width, hight) of the area affected by the render method
    fn render(&self, ctx: &mut Context<Self::Event>);

    /// Method which is used by the layout engine to get the size of a component
    /// by default the size will be calculated by using the render function with
    /// blocked drawing, but if speed matters this function can provide faster information
    #[inline(always)]
    fn size(&self) -> (f64, f64) {
        (0.0,0.0)
    }
}

///Ev: type of the event returned by the node constructed with this context
pub trait Context<Ev> where Ev: Sized{
    ///Add a new component, it will return an event, if it throws one
    ///the id has to be unique in this component
    fn add<F>(&mut self, _:Widget,Option<F>) where F: Fn(&Ev) {}

    ///this function registers a event listener, the closure is called
	///on every event
	///If variables in its scope should be used the enviroment has to be
	///captured by using the move keyword like so: move |event| ...
    fn on<F>(&mut self, _: F) where F: Fn(&Event) {}

    ///this function allows accessing the drawing context directly
    ///by default it does nothing
    fn draw<F>(&mut self, _: F) where F: Fn(&mut cairo::Context){}

    ///access the current position
    fn get_pos() -> (f64, f64){(0.0,0.0)}
}
