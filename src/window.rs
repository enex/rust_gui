//use time::precise_time_ns;

use std::any::Any;
use std::collections::HashMap;
use Context;
use App;
use ID;
use context::EventHandle;
use id::NULL_ID;
use Event;
use glutin;

/// Struct of a desktop app which is the basic setup
/// it includes everything nedded for a desktop application.
/// Sdl2 is used as the interface to the window manager and cairo for rendering.
pub struct Window{
	///delay betwen event checks default is 10
	pub delay: u32,
	///ǵlutin Window
	pub window: glutin::Window,
	///id of the currently drawing component
	//ctx: cairo::Context,//cairo context for drawing
	pub state: HashMap<ID, Box<Any + 'static>>,
	event_listener: HashMap<ID, Box<Fn(&Event, &mut EventHandle)+'static>>,
	event: Option<Event>,
	//id of the currently selected element
	pub focused: ID,
}

/// make library independen from Window just relai on builder

/// Events are handled with moved closures as filters that means
/// you can get a streeem of event form the Context and filert it
/// with a function like so:
///     ctx.events(move |event| ..., |event| ...)
/// whrere the first closure filters the closures so only interesting closures are lef
/// an the second calls the closure which does actual event handling inside of the component

impl Window{
	/// create a new window with given title, width and height
	pub fn new(w: glutin::Window) -> Window {
		
		Window{
			delay: 20,//12,
			window: w,
			//ctx: cr,
			state: HashMap::new(),
			event_listener: HashMap::new(),
			event: None,
			focused: NULL_ID,
		}
	}

	//TODO: combine show and app so that content is not duplicated
	//TODO: implement event propagation
	//TODO: only redraw if neccessary by tracking changes
	//TODO: only redraw changes and use caching

	/// function which takes the render function to generate the content, and then
	/// listens for input events it will return, if the window has been closed.
	pub fn show<F>(&mut self, render: F) where F:  Fn(&mut Context){
		//self.window.show();
		self.update();

		/*macro_rules! draw{//macro to draw
			($e:expr) => ({
				self.cairo_context().save();
				self.cairo_context().paint();
				{
					let mut c = $e;
					render(&mut c);
				}
				self.cairo_context().restore();
				self.update();
			});
			() => ({
				draw!(Context::new(self));
			});
		}

		//does event handling and redraw
		macro_rules! handle_event{
			() => ({
				for (id, ref f) in self.event_listener.iter(){
						let mut e = EventHandle::new(id, &mut self.state, &mut self.focused);
						(*f)(&self.event, &mut e);
				}
				draw!();
			})
		}

		draw!();

		'main : loop {
			'event : loop {
				self.event = sdl2::event::wait_event().unwrap();
				match self.event {
					Event::Quit{..} => break 'main,
					Event::KeyDown{
						timestamp: _,
						keycode: key,
						..
					} if key == sdl2::keycode::KeyCode::Escape => {
						break 'main
					},
					Event::None => break 'event,
					_ => handle_event!()//sdl2::time::delay(self.delay)
				}
			}
		}
		sdl2::quit();*/
	}

	/// function whic should be used to initialize an app. As the first parameter
	/// A Value implementing App should be given. The app will instatnly run
	/// after calling this function
	pub fn app<A>(&mut self, app: &mut A) where A:App{
		/*println!("lunch the app");
		self.update();
		//initial draw:
		self.cairo_context().save();
		{
			let mut c = Context::new(self);
			app.render(&mut c);
		}
		self.cairo_context().restore();
		self.update();

		'main : loop {
			'event : loop {
				match sdl2::event::poll_event() {
					Event::Quit{..} => break 'main,
					Event::KeyDown{
						timestamp: _,
						keycode: key,
						..
					} => {
						if key == sdl2::keycode::KeyCode::Escape {
							break 'main
						}
					},
					Event::None => break 'event,
					_ => sdl2::timer::delay(self.delay)
				}
			}
		}
		sdl2::quit();*/
	}

	/// search the state of a component by Id
	pub fn find_state<T>(&self, id: ID) -> Option<&T> where T:Any+'static{
		use state::UncheckedAnyRefExt;
		self.state.get(&id).map(|any|{
			unsafe{ (*any).downcast_ref_unchecked::<T>()}
		})
	}

	/// set the state
	pub fn set_state<T>(&mut self, id: ID, v: Box<T>) where T:Any+Clone+Eq+'static{
		self.state.insert(id, v as Box<Any + 'static>);
	}

	pub fn register_event_listener<F>(&mut self, id: ID, listener: Box<F>) where F: Fn(&Event, &mut EventHandle)+'static{
		self.event_listener.insert(id, listener);
	}

	/*
	/// get the ciro drawing context to draw on it
	#[stable]
	#[inline(always)]
	pub fn cairo_context(&mut self) -> &mut cairo::Context{
		&mut self.ctx
	}*/

	///Function to update view to cairo drawing
	#[stable]
	#[inline(always)]
	pub fn update(&mut self){
		//let _ = self.window.update_surface();//surface updaten
	}
}
