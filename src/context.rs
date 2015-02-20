use Window;
use Widget;
use Event;
use ID;
use id::NULL_ID;
use std::any::Any;
use std::collections::HashMap;
//use MouseCursor;

pub struct Context<'a>{
	window: &'a mut Window,
	depth: u8,
	id: ID,
	event: bool,
	x: f64,
	y: f64,
}

impl<'a> Context<'a>{
	pub fn new(window: &'a mut Window) -> Context<'a>{
		Context{
			window: window,
			depth: 0,
			id: NULL_ID,
			event: false,
			x: 0.,
			y: 0.,
		}
	}

	pub fn new_event(window: &'a mut Window) -> Context<'a>{
		Context{
			window: window,
			depth: 0,
			id: NULL_ID,
			event: true,
			x: 0.,
			y: 0.,
		}
	}

	/*/// this function allows accessing the drawing context directly
	/// by default it does nothing
	pub fn draw<F>(&mut self, draw: F) where F: Fn(&mut cairo::Context) {
		if !self.event{
			draw(self.window.cairo_context())
		}
	}*/

	/// Add a new component.
	/// The id has to be unique in this component
	#[inline(always)]
	pub fn add/*<F, Ev>*/(&mut self, id: u16, widget: &Widget/*, then: Option<F>*/) /*where F: Fn(&Ev)*/ {
		let mut nid = self.id;
		nid[self.depth as usize] = id;
		let mut c = Context{
			window: self.window,
			depth: self.depth+1,
			id: nid,
			event: self.event,
			x: self.x,
			y: self.y,
		};
		if c.depth > 11{
			panic!("the structure is to deep only a 12 child deep tree is allowed");
		}
		widget.render(&mut c);
	}

	/// Add a Component with given mutable state
	/// This is usefull if managing the whole application from the root component is
	/// Too complicated and splitting it to components is perefered.
	pub fn add_with_state(&mut self, id: u16, widget: &Widget){
		self.add(id, widget);
	}

	/// the id of the current component
	pub fn id(&self) -> ID{
		self.id
	}

	/// this function registers a event listener, the closure is called
	/// on every event it should decide if the event is relevant and then it should
	/// return true otherwhise it should do nothing
	/// If variables in its scope should be used the enviroment has to be
	/// captured by using the move keyword like so: move |event| ...
	pub fn on<F>(&mut self, filter: Box<F>) where F: Fn(&Event, &mut EventHandle)+'static {
		let s = self.id();
		//TODO: check if not already exsisting
		self.window.register_event_listener(s, filter)
	}

	/// returns true if the element is currently focused, if not it returns false
	pub fn focused(&self) -> bool{
		//println!("compare for focused: {:?} == {:?} => {}", self.window.focused, self.id, self.window.focused==self.id);
		self.window.focused == self.id
	}

	/// wether the element is hovered at the moment
	pub fn hover(&self) -> bool{
		false
	}

	/// get the state of the current component if it is already set,
	/// if not it returns none
	pub fn state<T>(&self) -> Option<&T> where T:Any+'static{
		use state::UncheckedAnyRefExt;
		self.window.state.get(&self.id).map(|any|{
			unsafe{ (*any).downcast_ref_unchecked::<T>()}
		})
	}

	/// move the current possition that newly added elements will be placed some
	/// at given position
	#[inline(always)]
	pub fn goto(&mut self, x: f64, y: f64){
		self.x = x;
		self.y = y;
		//self.window.cairo_context().translate(x,y)
	}

	///get the vurrent position
	pub fn pos(&mut self) -> (f64, f64){
		let mut x = 0.;
		let mut y = 0.;
		//self.window.cairo_context().user_to_device(&mut x, &mut y);
		(x, y)
	}
	pub fn pos_x(&mut self) -> f64{let (x,_) = self.pos(); x}
	pub fn pos_y(&mut self) -> f64{let (_,y) = self.pos(); y}
}

pub struct EventHandle<'a>{
	id: &'a ID,
	state: &'a mut HashMap<ID, Box<Any + 'static>>,
	focused: &'a mut ID,
}

impl<'a> EventHandle<'a>{
	pub fn new(id: &'a ID, state: &'a mut HashMap<ID, Box<Any + 'static>>, focused: &'a mut ID) -> EventHandle<'a>{
		EventHandle{
			id: id,
			state: state,
			focused: focused,
		}
	}

	//TODO: make shure the type is correct

	/// get the state of the component
	pub fn state<T>(&self) -> Option<&T> where T:Any+'static{
		use state::UncheckedAnyRefExt;
		self.state.get(self.id).map(|any|{
			unsafe{ (*any).downcast_ref_unchecked::<T>()}
		})
	}
	/// set the state of the component
	pub fn set_state<T>(&mut self, s: Box<T>) where T:Any+Eq+'static{
		self.state.insert(*self.id, s as Box<Any + 'static>);
	}
	/// get mutable reference to state of the widget
	pub fn mut_state<T>(&mut self) -> Option<&mut T> where T:Any+'static{
		use state::UncheckedAnyMutRefExt;
		self.state.get_mut(self.id).map(|any|{
			unsafe{ (*any).downcast_mut_unchecked::<T>()}
		})
	}

	/// emit an event for parent widgets
	pub fn emit(){

	}
	/// set the widget as focused
	pub fn focus(&mut self){
		*self.focused = *self.id
	}

	/// wether the current component is focused or not
	pub fn focused(&self) -> bool{
		*self.focused == *self.id
	}

	/*/// set the cursor to one of the system cursors.
	pub fn set_cursor(&mut self, cursor: SystemCursor){
		//use sdl2::mouse::show_cursor;
		//TODO: make this work somehow

		match Cursor::from_system(cursor){
			Ok(c) => c.set(),
			Err(e) => panic!("{}", e)
		};
	}*/
}
