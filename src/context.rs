use Widget;
use ID;
use glutin;
use draw;
use std::any::Any;
use state::State as AppState;
use std::marker::PhantomData;
use std::default::Default;

/// trait to register events
pub trait EventRegister{
	fn on_hover<F: Fn()>(&mut self, f: F){}

	/// register event listener for click event
	fn on_click(&mut self){}

	/// register event listener for key event
	fn on_key(&mut self){}
}

pub trait Context: EventRegister{
	fn add(){}
	fn with_event();
	fn draw_path<I:AsRef<[draw::PathInstr]>, V:AsRef<[f32]>>
			(&mut self, _: draw::Path<I,V>){}
	fn state();

	//event listening
}

/*pub struct Context<'a, Event, State>{
	/// how deep is the component this context belongs to
	depth: u8,
	/// id of the component the context belongs to
	id: ID,
	/// the application state
	state: &'a mut AppState,

	//PhantomData for Event and State to make it compile
	e: PhantomData<Event>,
	s: PhantomData<State>,
}

impl<'a, Event, State> Context<'a, Event, State>{
	pub fn new(state: &'a mut AppState) -> Context<'a, Event, State>{
		Context{
			depth: 0,
			id: [0; 12],
			state: state,
			e: PhantomData,
			s: PhantomData
		}
	}

	/// Add a new component.
	/// The id has to be unique in this component
	pub fn add<W,E,S>(&mut self, id: u16, widget: &W) where W:Widget<Event=E,State=S>{
		let mut nid = self.id;
		nid[self.depth as usize] = id;

		let mut c:Context<E,S> = Context{
			id: nid,
			depth: self.depth + 1,
			state: self.state,
			e: PhantomData,
			s: PhantomData
		};
		if c.depth > 11{
			panic!("the structure is to deep only a 12 child deep tree is allowed");
		}
		widget.render(&mut c);
	}

	/*/// add element at a given position
	pub fn add_at<E,S>(&mut self, id: u16, widget: &Widget<Event=E,State=S>, x: f32, y: f32) {
		println!("add_at x:{} y:{}",x,y);
		self.add(id, widget)
	}*/

	/// the id of the current component
	pub fn id(&self) -> ID{
		self.id
	}

	/// returns true if the element is currently focused, if not it returns false
	pub fn focused(&self) -> bool{
		self.state.focused == self.id
	}

	/// wether the element is hovered at the moment
	pub fn hovered(&self) -> bool{
		self.state.hovered == self.id
	}


	/// get the state of the current component, if it has not been set by
	/// an event it has the default value.
	pub fn state(&self) -> &State{
		use state::UncheckedAnyRefExt;
		unimplemented!();
		/*self.state.state.get(&self.id).map(|any|{
			unsafe{ (*any).downcast_ref_unchecked::<State>()}
		})*/
	}

	/// returns the default for every widget. This is also the way how
	/// theming is implemented. If the style sais this should have some
	/// specific parameters, then these are returned as a default.
	/// but theming is not jet implemented
	pub fn default<D:Default>(&self) -> D{
		Default::default()
	}

	/// get the current position
	pub fn pos(&mut self) -> (f64, f64){
		let x = 0.;
		let y = 0.;

		(x, y)
	}
	pub fn pos_x(&mut self) -> f64{let (x,_) = self.pos(); x}
	pub fn pos_y(&mut self) -> f64{let (_,y) = self.pos(); y}
}

/// event handling, closures can be registered to listen for certain events
/// they are called if this event happens. It is not possible to modify
/// a property in the closure, you can just modify the element state
/// and you can throw a event which is propagated to the parent element
impl <'a, Event, State> Context<'a, Event, State>{
	/// register event listener for hover event
	pub fn on_hover<F: Fn()>(&mut self, f: F){}

	/// register event listener for click event
	pub fn on_click(&mut self){}

	/// register event listener for key event
	pub fn on_key(&mut self){}
}


pub struct EventHandle<'a, Event, State>{
	id: &'a ID,
	state: &'a mut AppState,
	e: PhantomData<Event>,
	s: PhantomData<State>,
}

impl<'a, Event, State:'a> EventHandle<'a, Event, State>
		where State: Any + Default + 'static{
	pub fn new(id: &'a ID, state: &'a mut AppState) -> EventHandle<'a, Event, State>{
		EventHandle{
			id: id,
			state: state,
			e: PhantomData,
			s: PhantomData
		}
	}

	//TODO: make shure the type of state is correct and use the information given

	/// get the state of the component immutable
	pub fn state(&'a mut self) -> &'a State {
		self.state.get(&self.id)
	}
	/// get mutable reference to state of the widget
	pub fn mut_state(&'a mut self) -> &'a mut State {
		self.state.get_mut(&self.id)
	}

	/// emit an event for parent widgets
	pub fn emit(&mut self, e: Event){
		unimplemented!();
	}

	/// set the widget as focused, so keyboard events can be catched by this
	/// component or parent components
	pub fn focus(&mut self){
		self.state.focused = *self.id
	}

	/// wether the current component is focused or not
	pub fn focused(&self) -> bool{
		self.state.focused == *self.id
	}

	/// set the cursor to one of the system cursors.
	pub fn set_cursor(&mut self, cursor: glutin::MouseCursor){
		unimplemented!();
		//TODO: make this work somehow
	}
}*/
