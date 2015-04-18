use Widget;
use ID;
use glutin;
use draw;
use std::any::Any;
use state::State as AppState;
use std::marker::PhantomData;
use std::default::Default;
use Backend;

#[derive(Clone, Debug, Default, Copy)]
pub struct Pos{
	x: f32,
	y: f32,
}

/// trait to register events
pub trait EventRegister{
	type Event;
	type State: Default;

	/// register event listener for click event
	fn on_click<F:Fn(Pos, &mut EventHandle<Self::Event, Self::State>)>(&mut self, f: F){}

	/// register event listener for key event
	fn on_key(&mut self){}
}

pub trait Context: EventRegister{
	/// add a component
	fn add<W:Widget<State=S,Event=E>,S:Any+Default,E>(&mut self, id: u16, _: W){

	}
	/// add a component with coresponing position
	fn with_event(&mut self){}
	/// draw a path
	fn draw_path<I:AsRef<[draw::PathInstr]>, V:AsRef<[f32]>>
			(&mut self, _: draw::Path<I,V>){}

	/// get the current state
	fn state(&mut self) -> &Self::State;

	/// returns true if the element is currently focused, if not it returns false
	fn focused(&self) -> bool;

	/// wether the element is hovered at the moment
	fn hovered(&self) -> bool;

	/// the id of the current component
	fn id(&self) -> ID;

	/// returns the default for every widget. This is also the way how
	/// theming is implemented. If the style sais this should have some
	/// specific parameters, then these are returned as a default.
	/// but theming is not jet implemented
	fn default<D:Default>(&self) -> D{
		Default::default()
	}
}

/// context used to draw everything on screen
/// this will be passed to a component if this component should be drawn

//TODO: simplify
pub struct DrawContext<'a, D, Event, State> where D:'a{
	/// how deep is the component this context belongs to
	depth: u8,
	/// id of the component the context belongs to
	id: ID,
	/// the application state
	state: &'a mut AppState,
	be: &'a mut D,

	e: PhantomData<Event>,
	s: PhantomData<State>,
}

impl<'a, D:Backend, Event, State> DrawContext<'a, D, Event, State>{
	pub fn new(be: &'a mut D, state: &'a mut AppState) -> DrawContext<'a, D, Event, State>{
		DrawContext{
			depth: 0,
			id: [0; 12],
			state: state,
			be: be,

			e: PhantomData,
			s: PhantomData
		}
	}
}

impl<'a, D:Backend, Event, State:Default>EventRegister for DrawContext<'a, D, Event, State>{
	type Event = Event;
	type State = State;
}

impl<'a, D:Backend, Event, State:Any+Default>Context for DrawContext<'a, D, Event, State>{
	fn state(&mut self) -> &State{
		self.state.get(&self.id)
	}
	fn id(&self) -> ID{
		self.id
	}
	fn focused(&self) -> bool{
		self.state.focused == self.id
	}
	fn hovered(&self) -> bool{
		self.state.hovered == self.id
	}
	fn draw_path<I:AsRef<[draw::PathInstr]>, V:AsRef<[f32]>>
			(&mut self, path: draw::Path<I,V>){
		self.be.draw_path(path);
	}

	fn add<W:Widget<State=S,Event=E>,S:Any+Default,E>(&mut self, id: u16, w: W){
		let mut nid = self.id;
		nid[self.depth as usize] = id;

		println!("add: {:?} as {:?}", W::name(), nid);

		let mut c:DrawContext<D, E, S> = DrawContext{
			id: nid,
			depth: self.depth + 1,
			state: self.state,
			be: self.be,
			e: PhantomData,
			s: PhantomData
		};
		if c.depth > 11{
			panic!("the structure is to deep only a 12 child deep tree is allowed");
		}

		w.render(&mut c)
	}
}

/// this struct is used to handle events. Every registered event handler will
/// get a instance of this struct. This way it can for example propagate
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

	//TODO: make sure the type of state is correct and use the information given

	/// get the state of the component immutable
	pub fn state(&'a mut self) -> &'a State {
		self.state.get(&self.id)
	}

	/// get mutable reference to state of the widget this marks the widget as
	/// dirty and it will be rerendered.
	pub fn mut_state(&'a mut self) -> &'a mut State {
		self.state.get_mut(&self.id)
	}

	/// emit an event for parent widgets this event can then be catched by the
	/// parent and can be used to set the state of the widget or to propagate further
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
}
